#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod commands;
mod launchermeta;
pub(crate) mod structs;
use commands::*;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_app_path])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
