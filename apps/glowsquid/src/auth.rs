use once_cell::sync::OnceCell;
use tauri::{AppHandle, Manager, State};
use tokio_rusqlite::Connection;

#[derive(Clone)]
pub struct AuthState {
    /// If the user decided to enable authentication
    password_hash: OnceCell<String>,
    connection: Connection,
}

impl AuthState {
    pub async fn new(app_handle: &AppHandle) -> Self {
        let db_path = app_handle
            .path()
            .app_data_dir()
            .unwrap()
            .join("glowsquid.db");

        let connection = Connection::open(db_path)
            .await
            .expect("Failed to open database");

        Self {
            password_hash: OnceCell::new(),
            connection,
        }
    }
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
pub async fn start_add_account(state: State<'_, AuthState>) -> Result<String, ()> {
    todo!()
}
