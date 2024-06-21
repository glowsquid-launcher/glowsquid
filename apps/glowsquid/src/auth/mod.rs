pub mod server;
pub mod state;

use std::{future::IntoFuture, sync::Arc};

use axum::{routing, Router};
use copper::client::auth::MicrosoftAuthenticator;
use tauri::{async_runtime, AppHandle};
use tokio::net::TcpListener;

use error_stack::ResultExt;

use crate::error::Error;

// Glowsquids details. If you're making your own app, please do not use these.
// These are just for ease of use/packaging.
const CLIENT_ID: &str = "2aa32806-92e3-4242-babc-392ac0f0fd30";
const CLIENT_SECRET: &str = "nky8Q~8lORwTC0OjdxVsgZSs0hCdTcEdec3hNbaP";

#[tauri::command]
#[specta::specta]
/// Switch to a different account
///
/// # Returns
/// returns Ok(()) if the account was switched successfully, otherwise returns Err(())
pub async fn switch_to_account(
    state: tauri::State<'_, state::State>,
    account_index: u8,
) -> Result<(), ()> {
    if state.switch_to(account_index).await {
        Ok(())
    } else {
        Err(())
    }
}

#[tauri::command]
#[specta::specta]
/// Add a new account
///
/// # Returns
/// returns the index of the account that was added
pub async fn add_account(
    state: tauri::State<'_, state::State>,
) -> Result<u8, Error<server::AuthError>> {
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

    let token = recv_token.recv().await.unwrap()?;

    // shutdown the server
    server_task.abort();

    let profile = oauth
        .get_minecraft_profile(&token)
        .await
        .change_context(server::AuthError::MinecraftProfile)
        .map_err(Into::<Error<_>>::into)?;

    let idx = state.add_account(token, profile).await;

    Ok(idx)
}
