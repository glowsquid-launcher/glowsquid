use serde::Serialize;
use thiserror::Error;

use crate::{internal_errors::*, prisma::QueryError};

#[derive(Error, Debug, Serialize)]
pub enum AuthError {
  #[error("auth_error.cannot_parse_url(error={})", .0)]
  URLParseError(#[from] InternalParseError),
  #[error("auth_error.json_parse_error(error={})", .0)]
  JsonParseError(#[from] InternalJsonError),
  #[error("auth_error.request_error(error={})", .0)]
  RequestError(#[from] InternalRequestError),
  #[error("auth_error.account_not_found")]
  AccountNotFound,
  #[error("auth_error.database_error(error={})", .0)]
  DatabaseError(#[from] InternalQueryError),
  #[error("auth_error.missing_access_token")]
  MissingAccessToken,
  #[error("auth_error.missing_refresh_token")]
  MissingRefreshToken,
  #[error("auth_error.missing_uuid")]
  MissingUUID,
  #[error("auth_error.missing_expires_in")]
  MissingExpiresIn,
  #[error("auth_error.cannot_create_file_handle(error={})", .0)]
  IOError(#[from] InternalIoError),
  #[error("auth_error.cannot_open_in_browser")]
  CannotOpenInBrowser,
}

impl From<url::ParseError> for AuthError {
  fn from(err: url::ParseError) -> Self {
    AuthError::URLParseError(InternalParseError(err))
  }
}

impl From<std::io::Error> for AuthError {
  fn from(err: std::io::Error) -> Self {
    AuthError::IOError(InternalIoError(err))
  }
}

impl From<serde_json::Error> for AuthError {
  fn from(err: serde_json::Error) -> Self {
    AuthError::JsonParseError(InternalJsonError(err))
  }
}

impl From<reqwest::Error> for AuthError {
  fn from(err: reqwest::Error) -> Self {
    AuthError::RequestError(InternalRequestError(err))
  }
}

impl From<QueryError> for AuthError {
  fn from(err: QueryError) -> Self {
    AuthError::DatabaseError(InternalQueryError(err))
  }
}
