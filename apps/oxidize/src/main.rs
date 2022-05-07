#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod commands;
use commands::*;

pub const CLIENT_ID: &str = "2aa32806-92e3-4242-babc-392ac0f0fd30";

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_app_path])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
