use std::{error::Error, fmt::Display};

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
