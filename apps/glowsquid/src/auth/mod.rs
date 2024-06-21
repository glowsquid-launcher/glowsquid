pub mod server;

use std::{future::IntoFuture, sync::Arc};

use axum::{routing, Router};
use copper::client::auth::{MicrosoftAuthenticator, MinecraftProfile};
use once_cell::sync::OnceCell;
use tauri::{async_runtime, AppHandle, Manager, State};
use tokio::net::TcpListener;
use tokio_rusqlite::Connection;

use error_stack::ResultExt;

// Glowsquids details. If you're making your own app, please do not use these.
// These are just for ease of use/packaging.
const CLIENT_ID: &str = "2aa32806-92e3-4242-babc-392ac0f0fd30";
const CLIENT_SECRET: &str = "nky8Q~8lORwTC0OjdxVsgZSs0hCdTcEdec3hNbaP";

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

#[tauri::command]
#[specta::specta]
pub async fn add_account(
    state: State<'_, AuthState>,
    app: AppHandle,
) -> Result<MinecraftProfile, crate::error::Error<server::AuthError>> {
    // setup the server
    let listener = TcpListener::bind(("127.0.0.1", 0)).await.unwrap();
    let port = listener.local_addr().unwrap().port();

    let redirect_uri = format!("http://localhost:{}/code", port);

    // TODO: tell frontend that the user needs to go to the redirect uri and complete the auth process
    // for now, we'll just auto open it

    let (oauth, authentication_info) = MicrosoftAuthenticator::new(
        reqwest::Client::new(),
        redirect_uri,
        CLIENT_ID.to_string(),
        CLIENT_SECRET.to_string(),
    )
    .expect("To be able to create client");

    let (send_token, mut recv_token) = async_runtime::channel(1);

    let router = Router::new()
        .route("/code", routing::get(server::get_code))
        .with_state(server::AuthServerState {
            oauth: oauth.clone(),
            csrf_token: authentication_info.csrf_token,
            pkce_verifier: Arc::new(authentication_info.pkce_verifier),
            send_token,
        });

    let server_task =
        async_runtime::spawn(axum::serve(listener, router.into_make_service()).into_future());

    open::that(authentication_info.url.as_str()).expect("Failed to open browser");

    let minecraft_token = recv_token.recv().await.unwrap()?;

    // TODO: save token to database and update state

    // shutdown the server
    server_task.abort();

    oauth
        .get_minecraft_profile(&minecraft_token)
        .await
        .change_context(AuthError::MinecraftProfile)
        .map_err(Into::into)
}
