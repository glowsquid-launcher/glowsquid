use std::collections::HashMap;

use crate::{playerdb::get_profile, prisma::PrismaClient};
use prisma_client_rust::chrono::{Duration, Utc};
use reqwest::Url;
use serde::{Deserialize, Serialize};

use crate::{error::AuthError, prisma::account};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MinecraftProfile {
  pub uuid: String,
  pub access_token: String,
  pub refresh_token: String,
  pub expires_in: u32,
}

pub async fn process_adding_account(db: &PrismaClient, url: String) -> Result<(), AuthError> {
  let url = Url::parse(&url)?;
  let profile = create_profile_from_url(&url)?;
  let other_profile_info = get_profile(&profile.uuid).await?;

  let current_time = Utc::now();
  let expiry_time = current_time + Duration::seconds(profile.expires_in.into());

  db.account()
    .upsert(
      // search
      account::id::equals(profile.uuid.clone()),
      // create new
      (
        account::id::set(profile.uuid),
        account::username::set(other_profile_info.data.player.username.to_owned()),
        account::access_token::set(profile.access_token.to_owned()),
        account::refresh_token::set(profile.refresh_token.to_owned()),
        account::expires_at::set(expiry_time.into()),
        vec![],
      ),
      // update
      vec![
        account::username::set(other_profile_info.data.player.username.to_owned()),
        account::access_token::set(profile.access_token.to_owned()),
        account::refresh_token::set(profile.refresh_token.to_owned()),
        account::expires_at::set(expiry_time.into()),
      ],
    )
    .exec()
    .await?;

  Ok(())
}

pub fn create_profile_from_url(url: &Url) -> Result<MinecraftProfile, AuthError> {
  let mut query = url
    .query_pairs()
    .map(|(k, v)| (k.to_string(), v.to_string()))
    .collect::<HashMap<_, _>>();

  let uuid = query.remove("minecraftId").ok_or(AuthError::MissingUUID)?;
  let access_token = query
    .remove("minecraftToken")
    .ok_or(AuthError::MissingAccessToken)?;
  let refresh_token = query
    .remove("microsoftRefreshToken")
    .ok_or(AuthError::MissingRefreshToken)?;
  let expires_in = query
    .remove("microsoftExpiresIn")
    .ok_or(AuthError::MissingExpiresIn)?
    .parse::<u32>()
    .map_err(|_| AuthError::MissingExpiresIn)?;

  Ok(MinecraftProfile {
    uuid,
    access_token,
    refresh_token,
    expires_in,
  })
}

pub async fn refresh_account(
  db: &PrismaClient,
  account_id: &String,
  url: &Url,
) -> Result<(), AuthError> {
  let account = get_account(db, account_id).await?;

  let mut url = url.clone();
  url
    .query_pairs_mut()
    .append_pair("refreshToken", &account.refresh_token);

  let profile = reqwest::Client::new()
    .post(url)
    .send()
    .await?
    .json::<MinecraftProfile>()
    .await?;

  let current_time = Utc::now();
  let expiry_time = current_time + Duration::seconds(profile.expires_in.into());

  db.account()
    .find_unique(account::id::equals(account_id.to_owned()))
    .update(vec![
      account::access_token::set(profile.access_token),
      account::refresh_token::set(profile.refresh_token),
      account::expires_at::set(expiry_time.into()),
    ])
    .exec()
    .await?;

  Ok(())
}

pub async fn refresh_accounts(db: &PrismaClient, url: &Url) -> Result<(), AuthError> {
  let accounts = db.account().find_many(vec![]).exec().await?;
  let accounts_to_be_reloaded = accounts
    .into_iter()
    .filter(|account| account.expires_at < Utc::now())
    .collect::<Vec<_>>();

  for account in accounts_to_be_reloaded {
    refresh_account(db, &account.id, url).await?;
  }

  Ok(())
}

pub async fn get_account(db: &PrismaClient, id: &String) -> Result<MinecraftProfile, AuthError> {
  let account = db
    .account()
    .find_first(vec![account::id::equals(id.to_owned())])
    .exec()
    .await
    .map_err(|e| AuthError::DatabaseError(e.into()))?
    .ok_or(AuthError::AccountNotFound)?;

  Ok(MinecraftProfile {
    uuid: account.id,
    access_token: account.access_token,
    refresh_token: account.refresh_token,
    expires_in: (account.expires_at - account.last_refreshed).num_seconds() as u32,
  })
}
