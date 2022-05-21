use reqwest::Url;
use tauri::{
  api::{path::app_dir, shell::open},
  command, AppHandle, Manager,
};

use crate::{
  auth::{process_adding_account, refresh_account as refresh_account_impl},
  error::AuthError,
};

#[command]
pub fn get_app_path(app_handle: AppHandle) -> String {
  app_dir(&app_handle.config())
    .expect("Could not get app path")
    .join("glowsquid")
    .into_os_string()
    .into_string()
    .expect("Could not convert app path to string")
}

#[command]
pub async fn add_new_account(app_handle: AppHandle, dev: bool) -> Result<(), AuthError> {
  let (sender, reciever) = std::sync::mpsc::channel::<_>();
  println!("{}", app_dir(&app_handle.config()).unwrap().display());

  let app_handle_clone = app_handle.clone();
  let port = tauri_plugin_oauth::start(None, move |url| {
    sender
      .send(process_adding_account(
        url,
        app_dir(&app_handle_clone.config()).expect("Could not get app path"),
      ))
      .unwrap();
  })
  .unwrap();

  open(
    &app_handle.shell_scope(),
    if dev {
      format!("http://localhost:4000/api/auth/start/?port={}", port)
    } else {
      panic!("no production URL in place yet");
    },
    None,
  )
  .map_err(|_| AuthError::CannotOpenInBrowser)?;

  reciever.recv().unwrap()?;

  Ok(())
}

#[command]
// TODO: Error handling
pub async fn refresh_account(
  app_handle: AppHandle,
  dev: bool,
  account_id: String,
) -> Result<(), AuthError> {
  let app_path = app_dir(&app_handle.config()).unwrap();

  let url = if dev {
    "http://localhost:4000/api/auth/refresh/"
  } else {
    panic!("no production URL in place yet");
  };

  refresh_account_impl(account_id, app_path, Url::parse(url)?).await?;

  Ok(())
}
