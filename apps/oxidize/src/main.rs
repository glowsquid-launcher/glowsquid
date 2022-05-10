#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

pub(crate) mod commands;
pub(crate) mod auth;
use commands::*;

pub const CLIENT_ID: &str = "2aa32806-92e3-4242-babc-392ac0f0fd30";

fn main() {
  pretty_env_logger::init();

  tauri::Builder::default()
    .plugin(tauri_plugin_oauth::init())
    .invoke_handler(tauri::generate_handler![get_app_path, add_new_account])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
