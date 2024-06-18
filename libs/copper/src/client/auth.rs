use chrono::{DateTime, Duration, Utc};
use error_stack::{ensure, Report, Result, ResultExt};
use oauth2::{
    basic::{BasicClient, BasicTokenType},
    reqwest::async_http_client,
    url::{ParseError, Url},
    AccessToken, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken,
    EmptyExtraTokenFields, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope,
    StandardTokenResponse, TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{error::Error, fmt::Display};
use tracing::{debug, trace};
use veil::Redact;

use crate::api;

const AUTH_URL: &str = "https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize";
const TOKEN_URL: &str = "https://login.microsoftonline.com/consumers/oauth2/v2.0/token";

/// Information required for authentication
#[derive(Debug)]
pub struct AuthenticationInfo {
    pub url: Url,
    pub csrf_token: CsrfToken,
    pub pkce_verifier: PkceCodeVerifier,
}

/// Minecraft authentication client for authenticating with Microsoft services
///
/// This is the current implementation of the Microsoft authentication flowj
#[derive(Debug, Clone)]
pub struct MicrosoftAuthenticator(BasicClient, reqwest::Client);

impl MicrosoftAuthenticator {
    /// Create a new [`MSauth`] client.
    ///
    /// # Errors
    /// Errors if parsing the redirect uri fails
    pub fn new(
        reqwest_client: reqwest::Client,
        redirect_uri: String,
        client_id: String,
        client_secret: String,
    ) -> Result<(Self, AuthenticationInfo), ParseError> {
        debug!("Attempting to create new MSauth client");

        let client = BasicClient::new(
            ClientId::new(client_id),
            Some(ClientSecret::new(client_secret)),
            AuthUrl::new(AUTH_URL.to_string())?,
            Some(TokenUrl::new(TOKEN_URL.to_string())?),
        )
        .set_redirect_uri(RedirectUrl::new(redirect_uri)?);

        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        let (auth_url, csrf_token) = client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("XboxLive.signin".to_string()))
            .add_scope(Scope::new("offline_access".to_string()))
            .set_pkce_challenge(pkce_challenge)
            .url();

        let authentication_info = AuthenticationInfo {
            csrf_token,
            url: auth_url,
            pkce_verifier,
        };

        Ok((Self(client, reqwest_client), authentication_info))
    }

    #[tracing::instrument]
    async fn get_xbox_token(
        &self,
        access_token: &AccessToken,
    ) -> Result<api::auth::XboxResponse<api::auth::XboxLiveResponse>, XboxError> {
        debug!("Sending xbox live token request");
        let xbox_live_request = self
            .1
            .post("https://user.auth.xboxlive.com/user/authenticate")
            .json(&json!({
                "Properties": {
                    "AuthMethod": "RPS",
                    "SiteName": "user.auth.xboxlive.com",
                    "RpsTicket": format!("d={}", access_token.secret())
                },
                "RelyingParty": "http://auth.xboxlive.com",
                "TokenType": "JWT"
            }))
            .send()
            .await
            .change_context(XboxError::FetchError)?;

        trace!("Recieved {xbox_live_request:#?}");
        debug!("Parsing xbox live token request");

        xbox_live_request
            .json()
            .await
            .change_context(XboxError::DeserializeError)
    }

    #[tracing::instrument]
    async fn get_xsts_token(
        &self,
        xbox_response: &api::auth::XboxResponse<api::auth::XboxLiveResponse>,
    ) -> Result<api::auth::XboxResponse<api::auth::XstsResponse>, XboxError> {
        debug!("Sending xsts token request");

        let xsts_request = self
            .1
            .post("https://xsts.auth.xboxlive.com/xsts/authorize")
            .json(&json!({
                "Properties": {
                    "SandboxId": "RETAIL",
                    "UserTokens": [xbox_response.token()]
                },
                "RelyingParty": "rp://api.minecraftservices.com/",
                "TokenType": "JWT"
            }))
            .send()
            .await
            .attach_printable("Failed to send xsts request")
            .change_context(XboxError::FetchError)?;

        trace!("Recieved {xsts_request:#?}");
        debug!("Parsing xsts token request");

        xsts_request
            .json()
            .await
            .attach_printable("Failed to deserialize body")
            .change_context(XboxError::DeserializeError)
    }

    #[tracing::instrument]
    async fn get_minecraft_auth_token(
        &self,
        xsts_user_hash: &str,
        xsts_token: &str,
    ) -> Result<api::auth::MinecraftResponse, MinecraftTokenError> {
        debug!("Sending minecraft token request");

        let minecraft_token_request = self
            .1
            .post("https://api.minecraftservices.com/authentication/login_with_xbox")
            .json(&json!({
                "identityToken": format!("XBL3.0 x={};{}", xsts_user_hash, xsts_token),
            }))
            .send()
            .await
            .attach_printable("Failed to send minecraft token request")
            .change_context(MinecraftTokenError::FetchError)?;

        trace!("Recieved {minecraft_token_request:#?}");
        debug!("Parsing minecraft token request");

        minecraft_token_request
            .json::<api::auth::MinecraftResponse>()
            .await
            .attach_printable("Failed to deserialize body")
            .change_context(MinecraftTokenError::DeserializeError)
    }

    /// Get a minecraft token from a microsoft token
    ///
    /// # Errors
    /// Errors if the token is invalid or one of the requests fails.
    /// This can happen if the user does not own minecraft of if the token is expired
    #[tracing::instrument]
    pub async fn get_minecraft_token(
        &self,
        ms_token: StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    ) -> Result<MinecraftToken, MinecraftTokenError> {
        debug!("Attempting to get minecraft token from microsoft token");
        let access_token = ms_token.access_token();
        let xbox_live_response = self
            .get_xbox_token(access_token)
            .await
            .change_context(MinecraftTokenError::XboxLiveError)?;

        debug!("Recieved {xbox_live_response:#?}");

        let xbox_live_user_hash = &xbox_live_response
            .uhs()
            .ok_or(MinecraftTokenError::XboxLiveError)
            .attach_printable("No xui claims found")?;

        debug!("Recieved {xbox_live_user_hash:#?}");

        let xsts_response = self
            .get_xsts_token(&xbox_live_response)
            .await
            .change_context(MinecraftTokenError::XstsError)?;

        debug!("Recieved {xsts_response:#?}");

        let xsts_token = &xsts_response.token();
        let xsts_user_hash = &xsts_response
            .uhs()
            .ok_or(MinecraftTokenError::XstsError)
            .attach_printable("No xui claims found")?;

        ensure!(
            xsts_user_hash == xbox_live_user_hash,
            MinecraftTokenError::UhsMatchError
        );

        debug!("Sending minecraft token request");

        let minecraft_token_response = self
            .get_minecraft_auth_token(xsts_user_hash, xsts_token)
            .await?;

        debug!("token request response: {minecraft_token_response:#?}");
        debug!("Getting minecraft profile");

        let minecraft_profile_request = self
            .1
            .get("https://api.minecraftservices.com/minecraft/profile")
            .bearer_auth(&minecraft_token_response.access_token)
            .send()
            .await
            .attach_printable("Failed to send minecraft profile request")
            .change_context(MinecraftTokenError::FetchError)?;

        trace!("Recieved {minecraft_profile_request:#?}");

        let minecraft_profile_response = minecraft_profile_request
            .json::<api::auth::MinecraftProfile>()
            .await
            .attach_printable("Failed to deserialize body")
            .change_context(MinecraftTokenError::DeserializeError)?;

        Ok(MinecraftToken {
            access_token: minecraft_token_response.access_token,
            username: minecraft_profile_response.name,
            id: minecraft_profile_response.id,
            ms_token,
            expires_at: Utc::now()
                + Duration::try_seconds(minecraft_token_response.expires_in)
                    .expect("Failed to convert the expiry time into seconds."),
        })
    }

    /// Requests a microsoft access token
    ///
    /// # Errors
    /// Errors if:
    /// - The CSRF is invalid. This means that the code given by microsoft didn't match the internal one, and we aren't getting our information from the right place
    /// - The PKCE fails. This means that the code verifier was not the same as the one used to generate the code challenge, and we are not the ones who requested the token
    /// - The actual token request fails. This can happen if the code is invalid or expired
    #[tracing::instrument]
    pub async fn get_ms_access_token(
        &self,
        code: OauthCode,
        csrf: CsrfToken,
        pkce: &PkceCodeVerifier,
    ) -> Result<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>, OauthError> {
        ensure!(code.validate(&csrf), OauthError::InvalidCode);

        let pkce = PkceCodeVerifier::new(pkce.secret().to_string());
        self.0
            .exchange_code(code.into())
            .set_pkce_verifier(pkce)
            .request_async(async_http_client)
            .await
            .change_context(OauthError::TokenFetchError)
    }

    /// Gets the minecraft profile
    ///
    /// # Errors
    /// Errors if the token is invalid or the request fails.
    #[tracing::instrument]
    pub async fn get_minecraft_profile(
        &self,
        token: &MinecraftToken,
    ) -> Result<MinecraftProfile, reqwest::Error> {
        reqwest::Client::new()
            .get("https://api.minecraftservices.com/minecraft/profile")
            .bearer_auth(&token.access_token)
            .send()
            .await?
            .json()
            .await
            .map_err(Report::from)
    }

    /// Refreshes a microsoft access token
    ///
    /// # Errors
    /// Errors if the refresh token does not exist or the request fails.
    /// This can happen if the refresh token has been revoked by the user
    #[tracing::instrument]
    pub async fn refresh_ms_access_token(
        &self,
        response: &StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    ) -> Result<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>, OauthError> {
        let refresh_token = response
            .refresh_token()
            .ok_or(OauthError::InvalidCode)
            .attach_printable("No refresh token found")?;

        self.0
            .exchange_refresh_token(refresh_token)
            .request_async(async_http_client)
            .await
            .change_context(OauthError::TokenFetchError)
    }
}

#[derive(Debug)]
pub enum MinecraftTokenError {
    OauthError,
    XboxLiveError,
    XstsError,
    UhsMatchError,
    FetchError,
    DeserializeError,
}

impl Display for MinecraftTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::OauthError => "Error during oauth2 protocol",
            Self::XboxLiveError => "Error during xbox live protocol",
            Self::XstsError => "Error during xsts protocol",
            Self::UhsMatchError => "UHS mismatch between xsts and xbox live",
            Self::FetchError => "Error during fetch request",
            Self::DeserializeError => "Error during deserialization",
        })
    }
}
impl Error for MinecraftTokenError {}

#[derive(Debug)]
pub enum OauthError {
    InvalidCode,
    TokenFetchError,
}

impl Display for OauthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::InvalidCode => "CSRF codes do not match.",
            Self::TokenFetchError => "Error during oauth2 token fetch request.",
        })
    }
}

impl Error for OauthError {}

#[derive(Debug)]
pub enum XboxError {
    FetchError,
    DeserializeError,
}

impl Display for XboxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::FetchError => "Error during xbox live fetch request.",
            Self::DeserializeError => "Error during xbox live deserialization.",
        })
    }
}

impl Error for XboxError {}

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

/// A token that is used to login to minecraft
///
/// # Serialization
/// This token is serializable (in order to allow re-use similar to the official launcher),
/// _however_ if security is any concern, it is recommended to encrypt the struct before writing it to disk.
#[derive(Redact, Clone, Serialize, Deserialize)]
pub struct MinecraftToken {
    pub username: String,
    pub id: String,
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
    /// Checks if the current token is expired
    pub fn is_expired(&self) -> bool {
        self.expires_at < Utc::now()
    }

    /// Refreshes the token, returning an error if the refresh fails.
    ///
    /// # Errors
    /// Returns an error if the refresh fails.
    pub async fn refresh(&mut self, oauth: &MicrosoftAuthenticator) -> Result<(), RefreshError> {
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

#[derive(Serialize, Deserialize, Debug)]
pub struct MinecraftProfile {
    id: String,
    name: String,
    skins: Vec<Skin>,
    capes: Vec<Cape>,
}

impl MinecraftProfile {
    #[must_use]
    pub fn current_skin(&self) -> Option<&Skin> {
        self.skins
            .iter()
            .find(|skin| skin.state == UsageState::ACTIVE)
    }

    #[must_use]
    pub fn current_cape(&self) -> Option<&Cape> {
        self.capes
            .iter()
            .find(|cape| cape.state == UsageState::ACTIVE)
    }

    #[must_use]
    pub fn skin(&self, id: &str) -> Option<&Skin> {
        self.skins.iter().find(|skin| skin.id == id)
    }

    pub fn skins(&self) -> &[Skin] {
        &self.skins
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Skin {
    id: String,
    state: UsageState,
    url: String,
    variant: SkinVariant,
}

impl Skin {
    #[must_use]
    pub fn variant(&self) -> SkinVariant {
        self.variant
    }

    #[must_use]
    pub fn url(&self) -> &str {
        &self.url
    }

    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "UPPERCASE")]
pub enum SkinVariant {
    Classic,
    Slim,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cape {
    id: String,
    state: UsageState,
    url: String,
    alias: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "UPPERCASE")]
pub enum UsageState {
    ACTIVE,
    INACTIVE,
}
