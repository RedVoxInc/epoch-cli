//! Error types.

use std::error::Error;
use std::fmt;
use std::num::TryFromIntError;
use time::error::ComponentRange;

pub type Result<T> = std::result::Result<T, EpochError>;

/// A generic epoch error.
#[derive(Debug)]
pub struct EpochError {
    pub err: String,
}

impl EpochError {
    /// Used when an overflow or underflow occurs during time base conversions.
    pub fn numeric_precision(err: &str) -> Self {
        Self {
            err: format!("A numeric precision error occurred: {}", err),
        }
    }
}

impl fmt::Display for EpochError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "epoch-cli error: {}", self.err)
    }
}

impl Error for EpochError {}

/// Convert errors from the time crate to EpochErrors.
impl From<ComponentRange> for EpochError {
    fn from(err: ComponentRange) -> Self {
        Self {
            err: err.to_string(),
        }
    }
}

/// Convert errors during integer conversions to EpochErrors.
impl From<TryFromIntError> for EpochError {
    fn from(err: TryFromIntError) -> Self {
        Self {
            err: err.to_string(),
        }
    }
}
