// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod auth;
mod error;

use auth::*;
use tauri::{async_runtime::block_on, Manager};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
#[specta::specta]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let invoke_handler = {
        let builder = tauri_specta::ts::builder()
            .commands(tauri_specta::collect_commands![greet, add_account]);

        #[cfg(debug_assertions)] // <- Only export on non-release builds
        let builder = builder.path("../glowsquid-frontend/src/lib/bindings.ts");

        builder.build().unwrap()
    };

    tauri::Builder::default()
        .setup(|app| {
            app.manage(block_on(AuthState::new(app.app_handle())));

            Ok(())
        })
        .invoke_handler(invoke_handler)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
