use tauri::{api::path::app_dir, command, Config};

#[command]
pub fn get_app_path() -> String {
  app_dir(&Config::default())
    .expect("Could not get app path")
    .into_os_string()
    .into_string()
    .expect("Could not convert app path to string")
}
