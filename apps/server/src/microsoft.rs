use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use worker::console_log;

use crate::CLIENT_ID;

// types
pub enum Code {
    Code(String),
    RefreshToken(String),
}

#[derive(Deserialize, Debug)]
struct TokenResponse {
    access_token: String,
    refresh_token: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct XboxLiveRequestProperties {
    auth_method: String,
    site_name: String,
    rps_ticket: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct XboxLiveRequest {
    properties: XboxLiveRequestProperties,
    relaying_party: String,
    token_type: String,
}

// too lazy so I just quicktyped it
#[derive(Serialize, Deserialize, Debug)]
struct XboxLiveResponse {
    #[serde(rename = "IssueInstant")]
    issue_instant: String,
    #[serde(rename = "NotAfter")]
    not_after: String,
    #[serde(rename = "Token")]
    token: String,
    #[serde(rename = "DisplayClaims")]
    display_claims: DisplayClaims,
}

#[derive(Serialize, Deserialize, Debug)]
struct DisplayClaims {
    xui: Vec<Xui>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Xui {
    uhs: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct XSTSRequest {
    #[serde(rename = "Properties")]
    properties: XSTSProperties,
    #[serde(rename = "RelyingParty")]
    relying_party: String,
    #[serde(rename = "TokenType")]
    token_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct XSTSProperties {
    #[serde(rename = "SandboxId")]
    sandbox_id: String,
    #[serde(rename = "UserTokens")]
    user_tokens: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginWithXboxRequest {
    #[serde(rename = "identityToken")]
    identity_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginWithXboxResponse {
    username: String,
    roles: Vec<Option<serde_json::Value>>,
    access_token: String,
    token_type: String,
    expires_in: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProfileSuccessResponse {
    id: String,
    name: String,
    skins: Vec<ProfileSkin>,
    capes: Vec<Option<serde_json::Value>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProfileSkin {
    id: String,
    state: String,
    url: String,
    variant: String,
    alias: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProfileErrorResponse {
    path: String,
    #[serde(rename = "errorType")]
    error_type: String,
    error: String,
    #[serde(rename = "errorMessage")]
    error_message: String,
    #[serde(rename = "developerMessage")]
    developer_message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthenticateResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub uuid: String,
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AuthenticateError {
    InvalidProfile {
        error: String,
        error_message: String,
    },
    DeserializeError(String),
    RequestError(String),
}

// actual code

/// authenticates user
///
/// # Arguments
/// * `code` - the code the oauth flow returned or the refresh token
/// * `client_secret` - client secret gotten from enviorment
pub async fn authenticate_user(
    code: Code,
    client_secret: String,
    redirect_uri: &String,
) -> Result<AuthenticateResponse, AuthenticateError> {
    let client = reqwest::Client::new();

    let access_token_response =
        get_access_token(&client, &client_secret, &code, &redirect_uri).await?;
    console_log!("{:?}", access_token_response);
    let xbox_live_response = get_xbox_token(&client, &access_token_response.access_token).await?;
    console_log!("{:?}", xbox_live_response);
    let xsts_response = get_xsts_token(&client, xbox_live_response.token).await?;
    console_log!("{:?}", xsts_response);

    // sanity check
    if xbox_live_response.display_claims.xui[0].uhs != xsts_response.display_claims.xui[0].uhs {
        return Err(AuthenticateError::InvalidProfile {
            error: "uhs_missmatch".to_string(),
            error_message: "uhs mismatch".to_string(),
        });
    }

    let login_response = login_with_xbox(
        &client,
        xsts_response.display_claims.xui[0].uhs.to_owned(),
        xsts_response.token,
    )
    .await?;

    let profile_response = get_profile(&client, &login_response.access_token).await?;

    if profile_response.status() == StatusCode::OK {
        let profile_response = profile_response
            .json::<ProfileSuccessResponse>()
            .await
            .map_err(|err| AuthenticateError::DeserializeError(err.to_string()))?;

        Ok(AuthenticateResponse {
            access_token: login_response.access_token,
            refresh_token: access_token_response.refresh_token,
            uuid: xsts_response.display_claims.xui[0].uhs.to_owned(),
            username: profile_response.name,
        })
    } else {
        let profile_response = profile_response
            .json::<ProfileErrorResponse>()
            .await
            .map_err(|err| AuthenticateError::DeserializeError(err.to_string()))?;

        Err(AuthenticateError::InvalidProfile {
            error: profile_response.error,
            error_message: profile_response.error_message,
        })
    }
}

async fn get_access_token(
    client: &Client,
    client_secret: &String,
    code: &Code,
    redirect_uri: &String,
) -> Result<TokenResponse, AuthenticateError> {
    let req = client
        .post("https://login.live.com/oauth20_token.srf/")
        .form(&[
            ("client_id", CLIENT_ID),
            ("client_secret", client_secret),
            ("redirect_uri", redirect_uri),
            ("grant_type", "authorization_code"),
            match code {
                Code::Code(code) => ("code", code),
                Code::RefreshToken(token) => ("refresh_token", token),
            },
        ])
        .send()
        .await
        .map_err(|err| AuthenticateError::RequestError(err.to_string()))?;

    console_log!("{}", req.text().await.unwrap());
    // req.json::<TokenResponse>()
    // .await
    // .map_err(|err| AuthenticateError::DeserializeError(err.to_string()))
    todo!()
}

async fn get_xbox_token(
    client: &Client,
    access_token: &String,
) -> Result<XboxLiveResponse, AuthenticateError> {
    let request_properties = XboxLiveRequestProperties {
        auth_method: "RPS".to_string(),
        site_name: "user.auth.xboxlive.com".to_string(),
        rps_ticket: format!("d={}", access_token),
    };

    let request_body = XboxLiveRequest {
        properties: request_properties,
        relaying_party: "https://auth.xboxlive.com".to_string(),
        token_type: "JWT".to_string(),
    };

    client
        .post("https://user.auth.xboxlive.com/user/authenticate")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .json(&request_body)
        .send()
        .await
        .map_err(|err| AuthenticateError::RequestError(err.to_string()))?
        .json::<XboxLiveResponse>()
        .await
        .map_err(|err| AuthenticateError::DeserializeError(err.to_string()))
}

async fn get_xsts_token(
    client: &Client,
    token: String,
) -> Result<XboxLiveResponse, AuthenticateError> {
    let auth_properties = XSTSProperties {
        sandbox_id: "RETAIL".to_string(),
        user_tokens: vec![token],
    };

    let auth_request = XSTSRequest {
        properties: auth_properties,
        relying_party: "rp://api.minecraftservices.com/".to_string(),
        token_type: "JWT".to_string(),
    };

    client
        .post("https://xsts.auth.xboxlive.com/xsts/authorize")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .json(&auth_request)
        .send()
        .await
        .map_err(|err| AuthenticateError::RequestError(err.to_string()))?
        // responses are same
        .json::<XboxLiveResponse>()
        .await
        .map_err(|err| AuthenticateError::DeserializeError(err.to_string()))

    // TODO: error handling of custom errors
}

async fn login_with_xbox(
    client: &Client,
    uhs: String,
    token: String,
) -> Result<LoginWithXboxResponse, AuthenticateError> {
    let login_request = LoginWithXboxRequest {
        identity_token: format!("XBL3.0 x={};{}", uhs, token),
    };

    client
        .post("https://api.minecraftservices.com/authentication/login_with_xbox")
        .json(&login_request)
        .send()
        .await
        .map_err(|err| AuthenticateError::RequestError(err.to_string()))?
        .json::<LoginWithXboxResponse>()
        .await
        .map_err(|err| AuthenticateError::DeserializeError(err.to_string()))
}

async fn get_profile(
    client: &Client,
    access_token: &String,
) -> Result<reqwest::Response, AuthenticateError> {
    client
        .get("https://api.minecraftservices.com/profile")
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await
        .map_err(|err| AuthenticateError::RequestError(err.to_string()))
}
