// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod auth;

use auth::*;
use tauri::{async_runtime::block_on, Manager};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(block_on(AuthState::new(app.app_handle())));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, start_add_account])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
