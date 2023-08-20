use chrono::{Duration, Utc};
use error_stack::{ensure, IntoReport, Result, ResultExt};
use oauth2::{
    basic::{BasicClient, BasicTokenType},
    reqwest::async_http_client,
    url::{ParseError, Url},
    AccessToken, AuthUrl, ClientId, ClientSecret, CsrfToken, EmptyExtraTokenFields,
    PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope, StandardTokenResponse, TokenResponse,
    TokenUrl,
};
use serde_json::json;
use tracing::{debug, trace};

use self::structs::{
    MinecraftProfile, MinecraftResponse, OauthCode, XboxLiveResponse, XboxResponse, XstsResponse,
};
use self::{
    errors::{MinecraftTokenError, OauthError, XboxError},
    structs::MinecraftToken,
};

pub mod errors;
pub mod structs;

const AUTH_URL: &str = "https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize";
const TOKEN_URL: &str = "https://login.microsoftonline.com/consumers/oauth2/v2.0/token";

#[derive(Debug, Clone)]
pub struct MSauth(BasicClient, reqwest::Client);

impl MSauth {
    /// Create a new [`MSauth`] client.
    ///
    /// # Errors
    /// Errors if parsing the redirect uri fails
    pub fn new(
        redirect_uri: String,
        client_id: String,
        client_secret: String,
    ) -> Result<Self, ParseError> {
        debug!("Attempting to create new MSauth client");

        let client = BasicClient::new(
            ClientId::new(client_id),
            Some(ClientSecret::new(client_secret)),
            AuthUrl::new(AUTH_URL.to_string())?,
            Some(TokenUrl::new(TOKEN_URL.to_string())?),
        )
        .set_redirect_uri(RedirectUrl::new(redirect_uri)?);

        Ok(Self(client, reqwest::Client::new()))
    }

    pub fn get_auth_info(&self) -> (Url, CsrfToken, PkceCodeVerifier) {
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        let (auth_url, csrf_token) = self
            .0
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("XboxLive.signin".to_string()))
            .add_scope(Scope::new("offline_access".to_string()))
            .set_pkce_challenge(pkce_challenge)
            .url();

        (auth_url, csrf_token, pkce_verifier)
    }

    #[tracing::instrument]
    async fn get_xbox_token(
        &self,
        access_token: &AccessToken,
    ) -> Result<XboxResponse<XboxLiveResponse>, XboxError> {
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
            .into_report()
            .change_context(XboxError::FetchError)?;

        trace!("Recieved {xbox_live_request:#?}");
        debug!("Parsing xbox live token request");

        xbox_live_request
            .json()
            .await
            .into_report()
            .change_context(XboxError::DeserializeError)
    }

    #[tracing::instrument]
    async fn get_xsts_token(
        &self,
        xbox_response: &XboxResponse<XboxLiveResponse>,
    ) -> Result<XboxResponse<XstsResponse>, XboxError> {
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
            .into_report()
            .attach_printable("Failed to send xsts request")
            .change_context(XboxError::FetchError)?;

        trace!("Recieved {xsts_request:#?}");
        debug!("Parsing xsts token request");

        xsts_request
            .json()
            .await
            .into_report()
            .attach_printable("Failed to deserialize body")
            .change_context(XboxError::DeserializeError)
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

        trace!("Recieved {xbox_live_response:#?}");

        let xbox_live_user_hash = &xbox_live_response
            .uhs()
            .ok_or(MinecraftTokenError::XboxLiveError)
            .into_report()
            .attach_printable("No xui claims found")?;

        trace!("Recieved {xbox_live_user_hash:#?}");

        let xsts_response = self
            .get_xsts_token(&xbox_live_response)
            .await
            .change_context(MinecraftTokenError::XstsError)?;

        trace!("Recieved {xsts_response:#?}");

        let xsts_token = &xsts_response.token();
        let xsts_user_hash = &xsts_response
            .uhs()
            .ok_or(MinecraftTokenError::XstsError)
            .into_report()
            .attach_printable("No xui claims found")?;

        ensure!(
            xsts_user_hash == xbox_live_user_hash,
            MinecraftTokenError::UhsMatchError
        );

        debug!("Sending minecraft token request");

        let minecraft_request = self
            .1
            .post("https://api.minecraftservices.com/authentication/login_with_xbox")
            .json(&json!({
                "identityToken": format!("XBL3.0 x={};{}", xsts_user_hash, xsts_token),
            }))
            .send()
            .await
            .into_report()
            .attach_printable("Failed to send minecraft request")
            .change_context(MinecraftTokenError::FetchError)?;

        trace!("Recieved {minecraft_request:#?}");
        debug!("Parsing minecraft token request");

        let response = minecraft_request
            .json::<MinecraftResponse>()
            .await
            .into_report()
            .attach_printable("Failed to deserialize body")
            .change_context(MinecraftTokenError::DeserializeError)?;

        Ok(MinecraftToken {
            access_token: response.access_token,
            username: response.username,
            ms_token,
            expires_at: Utc::now() + Duration::seconds(response.expires_in),
        })
    }

    /// Requests a microsoft access token
    ///
    /// # Errors
    /// Errors if the code is invalid, the CSRF fails, the PKCE fails, or the request fails
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
            .into_report()
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
            .into_report()
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
            .into_report()
            .attach_printable("No refresh token found")?;

        self.0
            .exchange_refresh_token(refresh_token)
            .request_async(async_http_client)
            .await
            .into_report()
            .change_context(OauthError::TokenFetchError)
    }
}
