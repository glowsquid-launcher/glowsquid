use std::{error::Error, fmt::Display};

use crate::prisma::QueryError;

use serde::Serialize;

pub fn parse_error_to_i18n(err: &url::ParseError) -> String {
  match err {
    url::ParseError::EmptyHost => "empty_host",
    url::ParseError::IdnaError => "inda_error",
    url::ParseError::InvalidPort => "invalid_port",
    url::ParseError::InvalidIpv4Address => "invalid_ipv4_address",
    url::ParseError::InvalidIpv6Address => "invalid_ipv6_address",
    url::ParseError::InvalidDomainCharacter => "invalid_domain_character",
    url::ParseError::RelativeUrlWithoutBase => "relative_url_without_base",
    url::ParseError::RelativeUrlWithCannotBeABaseBase => "relative_url_with_cannot_be_a_base_base",
    url::ParseError::SetHostOnCannotBeABaseUrl => "set_host_on_cannot_be_a_base_url",
    url::ParseError::Overflow => "overflow",
    _ => "unknown_error",
  }
  .to_string()
}

#[derive(Debug)]
pub struct InternalParseError(pub url::ParseError);

impl Display for InternalParseError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", parse_error_to_i18n(&self.0))
  }
}

impl Serialize for InternalParseError {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.serialize_str(&parse_error_to_i18n(&self.0))
  }
}

impl Error for InternalParseError {}

#[derive(Debug)]
pub struct InternalIoError(pub std::io::Error);

impl Display for InternalIoError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0.kind())
  }
}

impl Serialize for InternalIoError {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.serialize_str(&self.0.kind().to_string())
  }
}

impl Error for InternalIoError {}

#[derive(Debug)]
pub struct InternalJsonError(pub serde_json::Error);

impl Display for InternalJsonError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl Serialize for InternalJsonError {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.serialize_str(&self.0.to_string())
  }
}

impl Error for InternalJsonError {}

#[derive(Debug)]
pub struct InternalRequestError(pub reqwest::Error);

impl Display for InternalRequestError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl Serialize for InternalRequestError {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.serialize_str(&self.0.to_string())
  }
}

impl From<reqwest::Error> for InternalRequestError {
  fn from(err: reqwest::Error) -> Self {
    InternalRequestError(err)
  }
}

impl Error for InternalRequestError {}

#[derive(Debug)]
pub struct InternalQueryError(pub QueryError);

impl Display for InternalQueryError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl Serialize for InternalQueryError {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.serialize_str(&self.0.to_string())
  }
}

impl From<QueryError> for InternalQueryError {
  fn from(err: QueryError) -> Self {
    InternalQueryError(err)
  }
}

impl Error for InternalQueryError {}
