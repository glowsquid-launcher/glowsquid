// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod auth;
pub mod error;
pub mod state;

use auth::*;
use specta::TypeCollection;
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
            .types(TypeCollection::default().register::<auth::state::AuthState>())
            .commands(tauri_specta::collect_commands![
                greet,
                add_account,
                switch_to_account
            ]);

        #[cfg(debug_assertions)] // <- Only export on non-release builds
        let builder = builder.path("../glowsquid-frontend/src/lib/bindings.ts");

        builder.build().unwrap()
    };

    tauri::Builder::default()
        .setup(|app| {
            let handle = app.app_handle().clone();
            app.listen("sync_ready", move |_| {
                if !handle.manage(block_on(auth::state::State::new(&handle))) {
                    println!("Syncing auth state");
                    block_on(handle.state::<auth::state::State>().sync());
                }
            });

            Ok(())
        })
        .invoke_handler(invoke_handler)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
