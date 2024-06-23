use copper::client::auth::{MicrosoftAuthenticator, MinecraftProfile, MinecraftToken};
use once_cell::sync::OnceCell;
use prqlx::query;
use serde::Serialize;
use specta::Type;
use sqlx::SqlitePool;
use tauri::{AppHandle, Manager};
use tokio::net::TcpListener;

use crate::database::DbState;

// Glowsquids details. If you're making your own app, please do not use these.
// These are just for ease of use/packaging.
const CLIENT_ID: &str = "2aa32806-92e3-4242-babc-392ac0f0fd30";
const CLIENT_SECRET: &str = "nky8Q~8lORwTC0OjdxVsgZSs0hCdTcEdec3hNbaP";

#[derive(Clone)]
pub struct State {
    /// If the user decided to enable authentication
    pub password_hash: OnceCell<String>,
    pub pool: SqlitePool,
    pub frontend_state: tauri_plugin_state::State<AuthState>,
    /// Oauth handler
    pub oauth: MicrosoftAuthenticator,
    /// Port to listen for the oauth redirect
    ///
    /// Gotten from an available socket on launch by the system
    ///
    /// TODO: figure out a better way to make sure that this is always available
    pub port: u16,
}

#[derive(Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
/// State of currently authenticated profiles
pub struct AuthState {
    /// All the profiles that the user has added
    profiles: Vec<MinecraftProfile>,
    /// All currently encrypted profiles usernames
    encrypted_profiles: Vec<String>,
    /// Index of the currently selected profile
    current_profile_index: Option<u8>,
}

impl State {
    pub async fn new(app_handle: &AppHandle) -> Self {
        let db_state: tauri::State<DbState> = app_handle.state();
        let pool = db_state.connection();

        let listener = TcpListener::bind(("127.0.0.1", 0)).await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let redirect_uri = format!("http://localhost:{}/code", port);

        let oauth = MicrosoftAuthenticator::new(
            reqwest::Client::new(),
            redirect_uri,
            CLIENT_ID.to_string(),
            CLIENT_SECRET.to_string(),
        )
        .expect("To be able to create client");

        let db_users = query!(
            "
            from auth
            select { username, id }
            "
        )
        .fetch_all(&pool)
        .await
        .expect("To be able to fetch users");

        let frontend_state = AuthState {
            profiles: vec![],
            encrypted_profiles: vec![],
            current_profile_index: None,
        };

        Self {
            password_hash: OnceCell::new(),
            pool,
            frontend_state: tauri_plugin_state::State::new(
                "auth",
                frontend_state,
                app_handle.clone(),
            ),
            oauth,
            port,
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
