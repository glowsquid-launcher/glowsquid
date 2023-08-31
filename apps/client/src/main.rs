// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crypto::hashes::{blake2b::Blake2b256, Digest};

/// Calculates the Blake2b from a String
fn hash_blake2b(input: &str) -> Vec<u8> {
    let mut hasher = Blake2b256::new();
    hasher.update(input.as_bytes());
    hasher.finalize().to_vec()
}

use specta::specta;

#[tauri::command]
#[specta]
fn test_connection() -> String {
    "Hello from the backend!!".to_string()
}

fn main() {
    if cfg!(debug_assertions) {
        use specta::collect_types;
        use tauri_specta::ts;

        ts::export(
            collect_types![test_connection],
            "../frontend/src/lib/bridge/bindings.ts",
        )
        .unwrap();
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_copper::init())
        .plugin(tauri_plugin_stronghold::Builder::new(hash_blake2b).build())
        .invoke_handler(tauri::generate_handler![test_connection])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
