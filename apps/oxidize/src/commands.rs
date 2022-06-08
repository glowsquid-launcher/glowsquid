use reqwest::Url;
use tauri::{
  api::{path::app_dir, shell::open},
  async_runtime::block_on,
  command, AppHandle, Manager,
};

use crate::{
  auth::{process_adding_account, refresh_account, refresh_accounts},
  error::AuthError,
  prisma::new_client,
  GlowState,
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

  let port = tauri_plugin_oauth::start(None, move |url| {
    let client = block_on(new_client()).unwrap();
    sender
      .send(block_on(process_adding_account(&client, url)))
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

  reciever.recv().unwrap()
}

#[command]
pub async fn reload_account(
  state: tauri::State<'_, GlowState>,
  dev: bool,
  account_id: String,
) -> Result<(), AuthError> {
  let url = if dev {
    "http://localhost:4000/api/auth/refresh/"
  } else {
    panic!("no production URL in place yet");
  };

  refresh_account(&state.db, &account_id, &Url::parse(url)?).await?;

  Ok(())
}

#[command]
pub async fn reload_accounts(
  state: tauri::State<'_, GlowState>,
  dev: bool,
) -> Result<(), AuthError> {
  let url = if dev {
    "http://localhost:4000/api/auth/refresh/"
  } else {
    panic!("no production URL in place yet");
  };

  refresh_accounts(&state.db, &Url::parse(url).unwrap()).await?;

  Ok(())
}
