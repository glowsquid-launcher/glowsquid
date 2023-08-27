use std::{error::Error, fmt::Display};

use chrono::{DateTime, Utc};
use error_stack::{Result, ResultExt};
use oauth2::{
    basic::BasicTokenType, AuthorizationCode, CsrfToken, EmptyExtraTokenFields,
    StandardTokenResponse,
};
use serde::Deserialize;
use veil::Redact;

use super::MSauth;

#[derive(Redact, Deserialize)]
pub struct OauthCode {
    #[redact]
    code: String,
    state: String,
}

impl OauthCode {
    #[must_use]
    pub fn validate(&self, csrf: &CsrfToken) -> bool {
        &self.state == csrf.secret()
    }
}

impl From<OauthCode> for AuthorizationCode {
    fn from(val: OauthCode) -> Self {
        Self::new(val.code)
    }
}

pub struct XboxLiveResponse;
pub struct XstsResponse;

#[derive(Redact, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct XboxResponse<T> {
    #[serde(skip)]
    marker: std::marker::PhantomData<T>,
    #[redact]
    token: String,
    display_claims: DisplayClaims,
}

impl<T> XboxResponse<T> {
    #[must_use]
    pub fn token(&self) -> &str {
        &self.token
    }

    #[must_use]
    pub fn uhs(&self) -> Option<&str> {
        self.display_claims.xui.first().map(|xui| xui.uhs.as_str())
    }
}

#[derive(Debug, Deserialize)]
struct DisplayClaims {
    xui: Vec<Xui>,
}

#[derive(Debug, Deserialize)]
struct Xui {
    uhs: String,
}

#[derive(Redact, Deserialize)]
pub(in crate::auth) struct MinecraftResponse {
    pub username: String,
    #[redact]
    pub access_token: String,
    pub expires_in: i64,
}

#[derive(Redact, Clone)]
pub struct MinecraftToken {
    pub username: String,
    #[redact]
    pub access_token: String,
    #[redact]
    pub ms_token: StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug)]
pub enum RefreshError {
    Oauth,
    MinecraftTokenError,
}

impl Error for RefreshError {}

impl Display for RefreshError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Oauth => "Oauth error",
            Self::MinecraftTokenError => "Minecraft Token error",
        })
    }
}

impl MinecraftToken {
    #[must_use]
    pub fn is_expired(&self) -> bool {
        self.expires_at < Utc::now()
    }

    /// Refreshes the token, returning an error if the refresh fails.
    ///
    /// # Errors
    /// Returns an error if the refresh fails.
    pub async fn refresh(&mut self, oauth: &MSauth) -> Result<(), RefreshError> {
        let new_ms_token = oauth
            .refresh_ms_access_token(&self.ms_token)
            .await
            .change_context(RefreshError::Oauth)?;

        *self = oauth
            .get_minecraft_token(new_ms_token)
            .await
            .change_context(RefreshError::MinecraftTokenError)?;

        Ok(())
    }
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct MinecraftProfile {
    id: String,
    name: String,
    skins: Vec<Skin>,
    capes: Vec<Option<serde_json::Value>>,
}

impl MinecraftProfile {
    #[must_use]
    pub fn id(&self) -> &str {
        self.id.as_ref()
    }
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct Skin {
    id: String,
    state: String,
    url: String,
    variant: String,
    alias: Option<String>,
}
