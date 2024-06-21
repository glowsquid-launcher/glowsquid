use crate::state;
use copper::client::auth::MinecraftProfile;
use once_cell::sync::OnceCell;
use serde::Serialize;
use specta::Type;
use tauri::{AppHandle, Manager};
use tokio_rusqlite::Connection;

#[derive(Clone)]
pub struct State {
    /// If the user decided to enable authentication
    password_hash: OnceCell<String>,
    connection: Connection,
    frontend_state: state::State<AuthState>,
}

#[derive(Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
/// The state that the frontend will have access to
pub struct AuthState {
    profiles: Vec<MinecraftProfile>,
    /// Index of the currently selected profile
    current_profile: Option<u8>,
}

impl State {
    pub async fn new(app_handle: &AppHandle) -> Self {
        let db_path = app_handle
            .path()
            .app_data_dir()
            .unwrap()
            .join("accounts.db");

        let connection = Connection::open(db_path)
            .await
            .expect("Failed to open database");

        // TODO: load profiles from database
        let frontend_state = AuthState {
            profiles: vec![],
            current_profile: None,
        };

        Self {
            password_hash: OnceCell::new(),
            connection,
            frontend_state: state::State::new("auth", frontend_state, app_handle.clone()),
        }
    }
}
