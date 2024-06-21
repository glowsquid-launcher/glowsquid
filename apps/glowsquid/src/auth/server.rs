use std::{error::Error, fmt, sync::Arc};

use axum::{extract, response::IntoResponse};
use copper::client::auth::{MicrosoftAuthenticator, MinecraftToken, OauthCode};
use oauth2::{CsrfToken, PkceCodeVerifier};
use serde::Serialize;
use specta::Type;
use tauri::async_runtime;

use error_stack::ResultExt;

#[derive(Debug, Clone, Copy, Type, Serialize)]
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
pub(super) struct AuthServerState {
    pub oauth: MicrosoftAuthenticator,
    pub csrf_token: CsrfToken,
    pub pkce_verifier: Arc<PkceCodeVerifier>,
    pub send_token: async_runtime::Sender<error_stack::Result<MinecraftToken, AuthError>>,
}

pub(super) async fn get_code(
    extract::Query(code): extract::Query<OauthCode>,
    extract::State(state): extract::State<AuthServerState>,
) -> Result<&'static str, impl IntoResponse> {
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
