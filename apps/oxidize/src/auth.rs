use std::{collections::HashMap, fs::File, io::Write, path::PathBuf};

use reqwest::Url;
use serde::{Deserialize, Serialize};

use crate::error::AuthError;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct MinecraftProfile {
  pub uuid: String,
  pub access_token: String,
  pub refresh_token: String,
}

pub fn process_adding_account(url: String, app_path: PathBuf) -> Result<(), AuthError> {
  let url = Url::parse(&url)?;
  let profile = create_profile_from_url(&url)?;

  let final_account_path = app_path
    .join("accounts")
    .join(format!("{}.json", &profile.uuid));

  let prefix = final_account_path.parent().unwrap();
  std::fs::create_dir_all(prefix)?;

  let mut file_handle = File::create(final_account_path)?;
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

fn save_profile_to_file(profile: MinecraftProfile, file: &mut File) -> Result<(), AuthError> {
  let serialized_profile = serde_json::to_string(&profile).unwrap();
  write!(file, "{}", serialized_profile)?;

  Ok(())
}

#[cfg(test)]
mod tests {
  use std::io::Read;

  use super::*;
  use tempfile::*;

  #[test]
  fn can_create_profile() {
    // KEYBOARD MASHING GO BRRRRRRRRR
    let url =
      "localhost:1234/cb?minecraftId=123&microsoftRefreshToken=4dsalkfjsaldgha.sdagkhdsg.ASfg214.56&minecraftToken=78sdafsadkfjlsad.sdafaSAFa.kfhldgshglkdhsag9";

    let expected = MinecraftProfile {
      uuid: "123".to_string(),
      refresh_token: "4dsalkfjsaldgha.sdagkhdsg.ASfg214.56".to_string(),
      access_token: "78sdafsadkfjlsad.sdafaSAFa.kfhldgshglkdhsag9".to_string(),
    };

    let result =
      create_profile_from_url(&Url::parse(url).unwrap()).expect("Could not create profile");

    assert_eq!(expected, result);
  }

  #[test]
  fn can_save_profile() {
    let profile = MinecraftProfile {
      uuid: "123".to_string(),
      refresh_token: "4dsalkfjsaldgha.sdagkhdsg.ASfg214.56".to_string(),
      access_token: "78sdafsadkfjlsad.sdafaSAFa.kfhldgshglkdhsag9".to_string(),
    };

    let mut write_handle = NamedTempFile::new().expect("could not create temp file");
    let mut read_handle = write_handle.reopen().expect("could not reopen temp file");

    save_profile_to_file(profile, write_handle.as_file_mut()).expect("Could not save profile");

    let mut result_str = String::new();
    read_handle
      .read_to_string(&mut result_str)
      .expect("Could not read profile");

    let expected = MinecraftProfile {
      uuid: "123".to_string(),
      refresh_token: "4dsalkfjsaldgha.sdagkhdsg.ASfg214.56".to_string(),
      access_token: "78sdafsadkfjlsad.sdafaSAFa.kfhldgshglkdhsag9".to_string(),
    };

    let result =
      serde_json::from_str::<MinecraftProfile>(&result_str).expect("Could not parse profile");

    assert_eq!(expected, result);
  }

  #[test]
  fn can_fully_process() {
    let url =
      "localhost:1234/cb?minecraftId=123&microsoftRefreshToken=4dsalkfjsaldgha.sdagkhdsg.ASfg214.56&minecraftToken=78sdafsadkfjlsad.sdafaSAFa.kfhldgshglkdhsag9";

    let app_path = tempdir().expect("Could not create temp dir");
    let path = app_path.path().to_path_buf();

    process_adding_account(url.to_string(), path.clone())
      .expect("Could not process adding account");

    let account_path = path.join("accounts").join("123.json");
    let mut file_handle = File::open(account_path).expect("Could not open account file");
    let mut result_str = String::new();

    file_handle
      .read_to_string(&mut result_str)
      .expect("Could not read account file");

    let result =
      serde_json::from_str::<MinecraftProfile>(&result_str).expect("Could not parse account file");

    let expected = MinecraftProfile {
      uuid: "123".to_string(),
      refresh_token: "4dsalkfjsaldgha.sdagkhdsg.ASfg214.56".to_string(),
      access_token: "78sdafsadkfjlsad.sdafaSAFa.kfhldgshglkdhsag9".to_string(),
    };

    assert_eq!(expected, result);
  }
}
