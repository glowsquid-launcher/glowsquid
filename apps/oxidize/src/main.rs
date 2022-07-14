#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use glowsquid::{commands::*, prisma, GlowState};
use tauri::async_runtime::Mutex;


fn main() {
  pretty_env_logger::init();

  tauri::Builder::default()
    .plugin(tauri_plugin_oauth::init())
    .manage(GlowState {
      db: tauri::async_runtime::block_on(async move { prisma::new_client().await.unwrap() }),
      is_dev: Mutex::new(false),
    })
    .invoke_handler(tauri::generate_handler![
      get_app_path,
      add_new_account,
      reload_account,
      reload_accounts,
      get_accounts,
      set_info
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
