use std::{
    error,
    fmt::{self, Debug},
};

use serde::Serialize;
use specta::{Any, Type};

/// A custom error type that wraps around a error-stack error, converting it from Report<T> to Error<T>
///
/// Also known as the most unholy error type known to crabkind
#[derive(Debug, Serialize, Type)]
pub struct Error<T: Type + Serialize + error::Error + Send + Sync + 'static> {
    error: T,
    #[specta(type = Any)]
    report: error_stack::Report<T>,
}

impl<T: Type + Serialize + error::Error + Send + Sync + 'static> error::Error for Error<T> {}

impl<T: Type + Serialize + error::Error + Send + Sync + 'static> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        std::fmt::Display::fmt(&self.report, f)
    }
}

// copy instead of clone to enforce cheapness
impl<T: Type + Serialize + Send + Sync + error::Error + Copy + 'static> From<error_stack::Report<T>>
    for Error<T>
{
    fn from(report: error_stack::Report<T>) -> Self {
        let error = *report.current_context();
        Self { error, report }
    }
}
