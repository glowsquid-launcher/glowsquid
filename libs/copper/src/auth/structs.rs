use std::{error::Error, fmt::Display};

use chrono::{DateTime, Utc};
use error_stack::{Result, ResultExt};
use oauth2::{
    basic::BasicTokenType, AuthorizationCode, CsrfToken, EmptyExtraTokenFields,
    StandardTokenResponse,
};
use serde::Deserialize;
use veil::Redact;

use super::MicrosoftAuthenticator;
