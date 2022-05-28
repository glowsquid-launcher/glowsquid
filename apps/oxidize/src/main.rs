#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

pub(crate) mod auth;
pub(crate) mod commands;
pub(crate) mod error;
pub(crate) mod internal_errors;
pub mod playerdb;
pub(crate) mod prisma;
use commands::*;

pub const CLIENT_ID: &str = "2aa32806-92e3-4242-babc-392ac0f0fd30";
pub struct GlowState {
  pub(crate) db: prisma::PrismaClient,
}

fn main() {
  pretty_env_logger::init();

  tauri::Builder::default()
    .plugin(tauri_plugin_oauth::init())
    .manage(GlowState {
      db: tauri::async_runtime::block_on(async move { prisma::new_client().await.unwrap() }),
    })
    .invoke_handler(tauri::generate_handler![
      get_app_path,
      add_new_account,
      reload_account,
      reload_accounts
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
