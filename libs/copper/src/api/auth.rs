use serde::Deserialize;
use veil::Redact;

pub struct XboxLiveResponse;
pub struct XstsResponse;

#[derive(Redact, Deserialize)]
pub(crate) struct MinecraftResponse {
    #[redact]
    pub access_token: String,
    pub expires_in: i64,
}

#[derive(Redact, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct XboxResponse<T> {
    #[serde(skip)]
    marker: std::marker::PhantomData<T>,
    #[redact]
    token: String,
    display_claims: DisplayClaims,
}

impl<T> XboxResponse<T> {
    #[must_use]
    pub(crate) fn uhs(&self) -> Option<&str> {
        self.display_claims.xui.first().map(|xui| xui.uhs.as_str())
    }

    pub(crate) fn token(&self) -> &str {
        &self.token
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

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct MinecraftProfile {
    pub id: String,
    pub name: String,
}
