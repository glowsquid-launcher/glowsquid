use std::{borrow::Cow, collections::HashMap};

use tauri::{
  api::{path::app_dir, shell::open},
  command, AppHandle, Config, Manager,
};

use url::Url;

use crate::auth::MinecraftProfile;

#[command]
pub fn get_app_path() -> String {
  app_dir(&Config::default())
    .expect("Could not get app path")
    .join("glowsquid")
    .into_os_string()
    .into_string()
    .expect("Could not convert app path to string")
}

#[command]
// TODO: proper error handling
pub async fn add_new_account(app_handle: AppHandle, dev: bool) -> Result<(), ()> {
  let port = tauri_plugin_oauth::start(None, |url| {
    let profile = create_account_from_url(&url);
    // TODO: save to file
  })
  .unwrap();

  open(
    &app_handle.shell_scope(),
    if dev {
      format!("localhost:4000/api/auth/start/?port={}", port)
    } else {
      panic!("no production URL in place yet");
    },
    None,
  )
  .unwrap();

  Ok(())
}

fn create_account_from_url(url: &str) -> MinecraftProfile {
  let url = Url::parse(url).unwrap();
  let params = url.query_pairs().collect::<HashMap<_, _>>();
  MinecraftProfile {
    uuid: params
      .get(&Cow::Borrowed("minecraftId"))
      .unwrap()
      .to_string(),
    refresh_token: params
      .get(&Cow::Borrowed("microsoftRefreshToken"))
      .unwrap()
      .to_string(),
    access_token: params
      .get(&Cow::Borrowed("microsoftAccessToken"))
      .unwrap()
      .to_string(),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn can_create_account() {
    // KEYBOARD MASHING GOOOOOOOO
    let url =
      "localhost:1234/cb?minecraftId=123&microsoftRefreshToken=4dsalkfjsaldgha.sdagkhdsg.ASfg214.56&microsoftAccessToken=78sdafsadkfjlsad.sdafaSAFa.kfhldgshglkdhsag9";

    let expected = MinecraftProfile {
      uuid: "123".to_string(),
      refresh_token: "4dsalkfjsaldgha.sdagkhdsg.ASfg214.56".to_string(),
      access_token: "78sdafsadkfjlsad.sdafaSAFa.kfhldgshglkdhsag9".to_string(),
    };

    let result = create_account_from_url(&url);

    assert_eq!(expected, result);
  }
}
