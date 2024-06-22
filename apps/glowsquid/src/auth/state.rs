use copper::client::auth::{MinecraftProfile, MinecraftToken};
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
    frontend_state: tauri_plugin_state::State<AuthState>,
}

#[derive(Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
/// State of currently authenticated profiles
pub struct AuthState {
    /// All the profiles that the user has added
    profiles: Vec<MinecraftProfile>,
    /// Index of the currently selected profile
    current_profile_index: Option<u8>,
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
            current_profile_index: None,
        };

        Self {
            password_hash: OnceCell::new(),
            connection,
            frontend_state: tauri_plugin_state::State::new(
                "auth",
                frontend_state,
                app_handle.clone(),
            ),
        }
    }

    pub async fn sync(&self) {
        self.frontend_state.sync().await;
    }

    pub async fn switch_to(&self, account_index: u8) -> bool {
        let mut state = self.frontend_state.lock().await;

        if account_index >= state.profiles.len() as u8 {
            return false;
        }

        state.current_profile_index = Some(account_index);

        true
    }

    pub async fn add_account(&self, token: MinecraftToken, profile: MinecraftProfile) -> u8 {
        let mut state = self.frontend_state.lock().await;

        // TODO: save token to database

        if let Some(idx) = state.profiles.iter().position(|p| p.id() == profile.id()) {
            return idx.try_into().unwrap();
        }
        state.profiles.push(profile);

        state.profiles.len() as u8 - 1
    }
}
