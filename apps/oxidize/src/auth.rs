use std::{collections::HashMap, fs::File, io::Write, path::PathBuf};

use reqwest::Url;
use serde::{Deserialize, Serialize};
use tokio::io::AsyncReadExt;

use crate::error::AuthError;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MinecraftProfile {
  pub uuid: String,
  pub access_token: String,
  pub refresh_token: String,
}

pub fn process_adding_account(url: String, app_path: PathBuf) -> Result<(), AuthError> {
  let url = Url::parse(&url)?;
  let profile = create_profile_from_url(&url)?;

  let account_path = app_path
    .join("accounts")
    .join(format!("{}.json", &profile.uuid));

  let prefix = account_path.parent().unwrap();
  std::fs::create_dir_all(prefix)?;

  let mut file_handle = File::create(account_path)?;
  save_profile_to_file(profile, &mut file_handle)?;

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

  Ok(MinecraftProfile {
    uuid,
    access_token,
    refresh_token,
  })
}

pub async fn refresh_account(
  account_id: String,
  app_path: PathBuf,
  url: Url,
) -> Result<(), AuthError> {
  let account = get_account(&account_id, app_path.clone()).await?;

  let mut url = url.clone();
  url
    .query_pairs_mut()
    .append_pair("refreshToken", &account.refresh_token);

  let profile = reqwest::Client::new()
    .post(url)
    .send()
    .await
    .unwrap()
    .json::<MinecraftProfile>()
    .await
    .unwrap();

  let file_handle = tokio::fs::File::create(
    app_path
      .join("accounts")
      .join(format!("{}.json", &account_id)),
  )
  .await?;
  save_profile_to_file(profile, &mut file_handle.into_std().await)?;

  Ok(())
}

pub async fn get_account(
  account_id: &String,
  app_path: PathBuf,
) -> Result<MinecraftProfile, AuthError> {
  let account_path = app_path
    .join("accounts")
    .join(format!("{}.json", account_id));

  let prefix = account_path.parent().unwrap();
  std::fs::create_dir_all(prefix)?;

  let mut account_string = String::new();
  tokio::fs::File::open(&account_path)
    .await?
    .read_to_string(&mut account_string)
    .await?;
  let account: MinecraftProfile = serde_json::from_str(&account_string)?;

  Ok(account)
}

fn save_profile_to_file(profile: MinecraftProfile, file: &mut File) -> Result<(), AuthError> {
  let serialized_profile = serde_json::to_string(&profile).unwrap();
  write!(file, "{}", serialized_profile)?;

  Ok(())
}

#[cfg(test)]
mod tests {
  use std::io::Read;
  use wiremock::{
    matchers::{method, path, query_param},
    Mock, MockServer, ResponseTemplate,
  };

  use super::*;
  use tempfile::*;

  const MINECRAFT_ID: &str = "b9c8f9c0-f8a3-4b5b-b8b6-f8f8f8f8f8f8";
  const DEFAULT_PORT: u16 = 4000;

  const REFRESH_TOKEN: &str =
    "dsklfajsdalkfj4213l4JKldsafa1243.0das8g8sddsafSAlfladskfjslad.j1ldfkajdee";
  const ACCESS_TOKEN: &str = "am123oNg.u5.dksjfa213Hlksafjlkajg2.jlkkJalfadf";

  const REFRESHED_REFRESH_TOKEN: &str = "jafkdsafl124.Jkagha241.Jlkh1lk35154";
  const REFRESHED_ACCESS_TOKEN: &str = "1dsaklfhafd3.l41s9.lkdsafhsaJ2h41k4h";

  #[test]
  fn can_create_profile() {
    let url = format!(
      "localhost:{}/cb?minecraftId={}&microsoftRefreshToken={}&minecraftToken={}",
      DEFAULT_PORT, MINECRAFT_ID, REFRESH_TOKEN, ACCESS_TOKEN
    );

    let test_profile: MinecraftProfile = MinecraftProfile {
      uuid: MINECRAFT_ID.to_string(),
      refresh_token: REFRESH_TOKEN.to_string(),
      access_token: ACCESS_TOKEN.to_string(),
    };

    let result =
      create_profile_from_url(&Url::parse(&url).unwrap()).expect("Could not create profile");

    assert_eq!(test_profile, result);
  }

  #[test]
  fn can_save_profile() {
    let test_profile: MinecraftProfile = MinecraftProfile {
      uuid: MINECRAFT_ID.to_string(),
      refresh_token: REFRESH_TOKEN.to_string(),
      access_token: ACCESS_TOKEN.to_string(),
    };

    let mut write_handle = NamedTempFile::new().expect("could not create temp file");
    let mut read_handle = write_handle.reopen().expect("could not reopen temp file");

    save_profile_to_file(test_profile.clone(), write_handle.as_file_mut())
      .expect("Could not save profile");

    let mut result_str = String::new();
    read_handle
      .read_to_string(&mut result_str)
      .expect("Could not read profile");

    let result =
      serde_json::from_str::<MinecraftProfile>(&result_str).expect("Could not parse profile");

    assert_eq!(test_profile, result);
  }

  #[test]
  fn can_create_account() {
    let test_profile: MinecraftProfile = MinecraftProfile {
      uuid: MINECRAFT_ID.to_string(),
      refresh_token: REFRESH_TOKEN.to_string(),
      access_token: ACCESS_TOKEN.to_string(),
    };

    let url = format!(
      "localhost:{}/cb?minecraftId={}&microsoftRefreshToken={}&minecraftToken={}",
      DEFAULT_PORT, MINECRAFT_ID, REFRESH_TOKEN, ACCESS_TOKEN
    );

    let app_path = tempdir().expect("Could not create temp dir");
    let path = app_path.path().to_path_buf();

    process_adding_account(url, path.clone()).expect("Could not process adding account");

    let account_path = path
      .join("accounts")
      .join(format!("{}.json", test_profile.uuid));
    let mut file_handle = File::open(account_path).expect("Could not open account file");
    let mut result_str = String::new();

    file_handle
      .read_to_string(&mut result_str)
      .expect("Could not read account file");

    let result =
      serde_json::from_str::<MinecraftProfile>(&result_str).expect("Could not parse account file");

    assert_eq!(test_profile, result);
  }

  #[tokio::test]
  async fn can_get_account() {
    let test_profile: MinecraftProfile = MinecraftProfile {
      uuid: MINECRAFT_ID.to_string(),
      refresh_token: REFRESH_TOKEN.to_string(),
      access_token: ACCESS_TOKEN.to_string(),
    };

    let app_path = tempdir().expect("Could not create temp dir");
    let path = app_path.path().to_path_buf();

    let account_path = path
      .join("accounts")
      .join(format!("{}.json", test_profile.uuid));

    let prefix = account_path.parent().unwrap();
    tokio::fs::create_dir_all(prefix)
      .await
      .expect("Could not create dir");

    let file_handle = tokio::fs::File::create(account_path)
      .await
      .expect("Could not create account file");

    save_profile_to_file(test_profile.clone(), &mut file_handle.into_std().await)
      .expect("Could not save profile");

    let result = get_account(&test_profile.uuid.to_owned(), path.clone())
      .await
      .expect("Could not get account");

    assert_eq!(test_profile, result);
  }

  #[tokio::test]
  async fn can_refresh_account() {
    let server_uri = mock_server().await;

    let test_profile = MinecraftProfile {
      uuid: MINECRAFT_ID.to_string(),
      refresh_token: REFRESH_TOKEN.to_string(),
      access_token: ACCESS_TOKEN.to_string(),
    };

    let refreshed_profile = MinecraftProfile {
      uuid: MINECRAFT_ID.to_string(),
      refresh_token: REFRESHED_REFRESH_TOKEN.to_string(),
      access_token: REFRESHED_ACCESS_TOKEN.to_string(),
    };

    let app_path = tempdir().expect("Could not create temp dir");
    let path = app_path.path().to_path_buf();

    let account_path = path
      .join("accounts")
      .join(format!("{}.json", test_profile.uuid));

    let prefix = account_path.parent().unwrap();
    tokio::fs::create_dir_all(prefix)
      .await
      .expect("Could not create dir");

    let save_file_handle = tokio::fs::File::create(account_path.clone())
      .await
      .expect("Could not create account file");

    save_profile_to_file(test_profile.clone(), &mut save_file_handle.into_std().await)
      .expect("Could not save profile");

    refresh_account(
      test_profile.uuid.to_owned(),
      path.clone(),
      Url::parse(&format!("{}/api/auth/refresh", &server_uri)).expect("Could not parse url"),
    )
    .await
    .expect("Could not refresh account");

    let mut read_file_handle = tokio::fs::File::open(account_path)
      .await
      .expect("Could not open account file");

    let mut result_str = String::new();

    read_file_handle
      .read_to_string(&mut result_str)
      .await
      .expect("Could not read account file");

    let result =
      serde_json::from_str::<MinecraftProfile>(&result_str).expect("Could not parse account file");

    assert_eq!(refreshed_profile, result);
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
      }))
      .mount(&mock_server)
      .await;

    mock_server.uri()
  }
}
