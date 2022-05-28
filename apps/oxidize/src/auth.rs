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

pub async fn process_adding_account(db: PrismaClient, url: String) -> Result<(), AuthError> {
  let url = Url::parse(&url)?;
  let profile = create_profile_from_url(&url)?;
  let other_profile_info = get_profile(profile.uuid.clone()).await?;

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

fn create_profile_from_url(url: &Url) -> Result<MinecraftProfile, AuthError> {
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

#[cfg(test)]
mod tests {
  use wiremock::{
    matchers::{method, path, query_param},
    Mock, MockServer, ResponseTemplate,
  };

  use crate::prisma::new_client;

  use super::*;

  /// This is TNT_Man1671's profile (AKA Suyashtnt)
  const MINECRAFT_ID: &str = "52ddf2f1-a59f-4a19-822f-a6157f705320";
  const DEFAULT_PORT: u16 = 4000;

  const REFRESH_TOKEN: &str =
    "dsklfajsdalkfj4213l4JKldsafa1243.0das8g8sddsafSAlfladskfjslad.j1ldfkajdee";
  const ACCESS_TOKEN: &str = "am123oNg.u5.dksjfa213Hlksafjlkajg2.jlkkJalfadf";
  const EXPIRES_IN: u32 = 3600;

  const REFRESHED_REFRESH_TOKEN: &str = "jafkdsafl124.Jkagha241.Jlkh1lk35154";
  const REFRESHED_ACCESS_TOKEN: &str = "1dsaklfhafd3.l41s9.lkdsafhsaJ2h41k4h";

  #[test]
  fn can_create_profile() {
    let url = format!(
      "localhost:{}/cb?minecraftId={}&microsoftRefreshToken={}&minecraftToken={}&microsoftExpiresIn={}",
      DEFAULT_PORT, MINECRAFT_ID, REFRESH_TOKEN, ACCESS_TOKEN, EXPIRES_IN
    );

    let test_profile: MinecraftProfile = MinecraftProfile {
      uuid: MINECRAFT_ID.to_string(),
      refresh_token: REFRESH_TOKEN.to_string(),
      access_token: ACCESS_TOKEN.to_string(),
      expires_in: EXPIRES_IN,
    };

    let result =
      create_profile_from_url(&Url::parse(&url).unwrap()).expect("Could not create profile");

    assert_eq!(test_profile, result);
  }

  #[tokio::test]
  async fn can_create_account() {
    let test_profile: MinecraftProfile = MinecraftProfile {
      uuid: MINECRAFT_ID.to_string(),
      refresh_token: REFRESH_TOKEN.to_string(),
      access_token: ACCESS_TOKEN.to_string(),
      expires_in: EXPIRES_IN,
    };

    let url = format!(
      "localhost:{}/cb?minecraftId={}&microsoftRefreshToken={}&minecraftToken={}&microsoftExpiresIn={}",
      DEFAULT_PORT, MINECRAFT_ID, REFRESH_TOKEN, ACCESS_TOKEN, EXPIRES_IN
    );

    let db = new_client().await.unwrap();
    process_adding_account(db, url)
      .await
      .expect("Could not process adding account");

    let db = new_client().await.unwrap();

    let result = get_account(&db, &MINECRAFT_ID.to_string()).await.unwrap();

    assert_eq!(test_profile.uuid, result.uuid);
    assert_eq!(test_profile.refresh_token, result.refresh_token);
    assert_eq!(test_profile.access_token, result.access_token);
    assert!(test_profile.expires_in + 2 >= EXPIRES_IN);
  }

  #[tokio::test]
  async fn can_get_account() {
    let test_profile: MinecraftProfile = MinecraftProfile {
      uuid: MINECRAFT_ID.to_string(),
      refresh_token: REFRESH_TOKEN.to_string(),
      access_token: ACCESS_TOKEN.to_string(),
      expires_in: EXPIRES_IN,
    };

    let db = new_client().await.unwrap();

    let result = get_account(&db, &MINECRAFT_ID.to_owned()).await.unwrap();

    assert_eq!(test_profile.uuid, result.uuid);
    assert_eq!(test_profile.refresh_token, result.refresh_token);
    assert_eq!(test_profile.access_token, result.access_token);
    assert!(test_profile.expires_in + 2 >= EXPIRES_IN);
  }

  #[tokio::test]
  async fn can_refresh_account() {
    let server_uri = mock_server().await;

    let test_profile = MinecraftProfile {
      uuid: MINECRAFT_ID.to_string(),
      refresh_token: REFRESH_TOKEN.to_string(),
      access_token: ACCESS_TOKEN.to_string(),
      expires_in: EXPIRES_IN,
    };

    let refreshed_profile = MinecraftProfile {
      uuid: MINECRAFT_ID.to_string(),
      refresh_token: REFRESHED_REFRESH_TOKEN.to_string(),
      access_token: REFRESHED_ACCESS_TOKEN.to_string(),
      expires_in: EXPIRES_IN,
    };

    refresh_account(
      &new_client().await.unwrap(),
      &test_profile.uuid,
      &Url::parse(&format!("{}/api/auth/refresh", &server_uri)).expect("Could not parse url"),
    )
    .await
    .expect("Could not refresh account");

    let result = get_account(&new_client().await.unwrap(), &MINECRAFT_ID.to_owned())
      .await
      .unwrap();

    assert_eq!(refreshed_profile.uuid, result.uuid);
    assert_eq!(refreshed_profile.refresh_token, result.refresh_token);
    assert_eq!(refreshed_profile.access_token, result.access_token);
    assert!(result.expires_in + 2 >= EXPIRES_IN);
  }

  async fn mock_server() -> String {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
      .and(path("/api/auth/refresh"))
      .and(query_param("refreshToken", REFRESH_TOKEN))
      .respond_with(ResponseTemplate::new(200).set_body_json(MinecraftProfile {
        uuid: MINECRAFT_ID.to_string(),
        refresh_token: REFRESHED_REFRESH_TOKEN.to_string(),
        access_token: REFRESHED_ACCESS_TOKEN.to_string(),
        expires_in: EXPIRES_IN,
      }))
      .mount(&mock_server)
      .await;

    mock_server.uri()
  }
}
