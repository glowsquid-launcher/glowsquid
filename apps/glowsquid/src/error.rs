use std::{
    error,
    fmt::{self, Debug},
};

use serde::Serialize;
use serde_json::Value;
use specta::Type;

/// A custom error type that wraps around a error-stack error, converting it from Report<T> to Error<T>
///
/// Also known as the most unholy error type known to crabkind
#[derive(Debug, Serialize, Type)]
pub struct Error<T> {
    error: T,
    report: Value,
    #[serde(skip)]
    error_stack: error_stack::Report<T>,
}

impl<T: error::Error> error::Error for Error<T> {}

impl<T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        std::fmt::Display::fmt(&self.error_stack, f)
    }
}

// copy instead of clone to enforce cheapness
impl<T: Send + Sync + error::Error + Copy + 'static> From<error_stack::Report<T>> for Error<T> {
    fn from(error_stack: error_stack::Report<T>) -> Self {
        let error = *error_stack.current_context();
        Self {
            error,
            // cursed way to do this without cloning
            report: serde_json::from_str(&serde_json::to_string(&error_stack).unwrap()).unwrap(),
            error_stack,
        }
    }
}
