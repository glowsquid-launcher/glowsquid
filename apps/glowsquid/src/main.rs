// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod auth;
pub mod database;
pub mod error;

use auth::*;
use specta::TypeCollection;
use sqlx::SqlitePool;
use tauri::{async_runtime::block_on, Manager};

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
                remove_account,
                switch_to_account
            ]);

        #[cfg(debug_assertions)] // <- Only export on non-release builds
        let builder = builder.path("../glowsquid-frontend/src/lib/bindings.ts");

        builder.build().unwrap()
    };

    tauri::Builder::default()
        .setup(|app| {
            let data_path = app.path().app_data_dir()?;
            let db_path = data_path.join("glowsquid.db3");

            if !db_path.exists() {
                std::fs::File::create(&db_path)?;
            }

            let db = block_on(async move { SqlitePool::connect(db_path.to_str().unwrap()).await })?;

            let db_state = block_on(database::DbState::new(db))?;

            app.manage(db_state);

            Ok(())
        })
        .plugin(tauri_plugin_state::init(|app| {
            let handle = app.app_handle().clone();
            if !handle.manage(block_on(auth::state::State::new(&handle))) {
                block_on(handle.state::<auth::state::State>().sync());
            }
        }))
        .invoke_handler(invoke_handler)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
