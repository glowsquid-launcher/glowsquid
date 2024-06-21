use std::{
    error::Error,
    fmt,
    future::IntoFuture,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};

use axum::{extract, response::IntoResponse, routing, Router};
use copper::client::auth::{MicrosoftAuthenticator, MinecraftProfile, MinecraftToken, OauthCode};
use oauth2::{CsrfToken, PkceCodeVerifier};
use once_cell::sync::OnceCell;
use tauri::{async_runtime, AppHandle, Manager, State};
use tokio::net::TcpListener;
use tokio_rusqlite::Connection;

use error_stack::{Result, ResultExt};

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

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
pub async fn add_account(
    state: State<'_, AuthState>,
    app: AppHandle,
) -> Result<MinecraftProfile, AuthError> {
    // setup the server
    let listener = TcpListener::bind((
        "127.0.0.1",
        get_available_port().expect("Failed to get available port"),
    ))
    .await
    .unwrap();
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
        .route("/code", routing::get(get_code))
        .with_state(AuthServerState {
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
}

#[derive(Debug)]
pub enum AuthError {
    MsToken,
    MinecraftToken,
    MinecraftProfile,
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthError::MsToken => f.write_str("Failed to get Microsoft token"),
            AuthError::MinecraftToken => f.write_str("Failed to get Minecraft token"),
            AuthError::MinecraftProfile => f.write_str("Failed to get Minecraft profile"),
        }
    }
}

impl Error for AuthError {}

#[derive(Clone)]
struct AuthServerState {
    oauth: MicrosoftAuthenticator,
    csrf_token: CsrfToken,
    pkce_verifier: Arc<PkceCodeVerifier>,
    send_token: async_runtime::Sender<Result<MinecraftToken, AuthError>>,
}

async fn get_code(
    extract::Query(code): extract::Query<OauthCode>,
    extract::State(state): extract::State<AuthServerState>,
) -> std::result::Result<&'static str, impl IntoResponse> {
    let ms_token = match state
        .oauth
        .get_ms_access_token(code, state.csrf_token, &state.pkce_verifier)
        .await
        .change_context(AuthError::MsToken)
    {
        Ok(token) => token,
        Err(e) => {
            state.send_token.send(Err(e)).await.unwrap();
            return Err((
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get Microsoft token! Check glowsquid for extra information and to report this issue.",
            ));
        }
    };

    let token = match state
        .oauth
        .get_minecraft_token(ms_token.clone())
        .await
        .change_context(AuthError::MinecraftToken)
    {
        Ok(token) => token,
        Err(e) => {
            state.send_token.send(Err(e)).await.unwrap();
            return Err((
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get Minecraft token! Check glowsquid for extra information and to report this issue.",
            ));
        }
    };

    state.send_token.send(Ok(token)).await.unwrap();

    Ok("Authenticated! You can now close this window and go back to Glowsquid.")
}

fn get_available_port() -> Option<u16> {
    (8000..9000).find(|port| port_is_available(*port))
}

fn port_is_available(port: u16) -> bool {
    std::net::TcpListener::bind(("127.0.0.1", port)).is_ok()
}
