use serde::Serialize;
use thiserror::Error;

use crate::internal_errors::{InternalIoError, InternalParseError};

#[derive(Error, Debug, Serialize)]
pub enum AuthError {
  #[error("auth_error.cannot_parse_url(error={})", .0)]
  ParseError(#[from] InternalParseError),
  #[error("auth_error.missing_access_token")]
  MissingAccessToken,
  #[error("auth_error.missing_refresh_token")]
  MissingRefreshToken,
  #[error("auth_error.missing_uuid")]
  MissingUUID,
  #[error("auth_error.cannot_create_file_handle(error={})", .0)]
  CannotCreateFileHandle(#[from] InternalIoError),
}

impl From<url::ParseError> for AuthError {
  fn from(err: url::ParseError) -> Self {
    AuthError::ParseError(InternalParseError(err))
  }
}

impl From<std::io::Error> for AuthError {
  fn from(err: std::io::Error) -> Self {
    AuthError::CannotCreateFileHandle(InternalIoError(err))
  }
}
