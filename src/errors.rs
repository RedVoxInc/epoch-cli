use std::error::Error;
use std::fmt;
use std::num::TryFromIntError;
use time::error::ComponentRange;

pub type Result<T> = std::result::Result<T, EpochError>;

#[derive(Debug)]
pub struct EpochError {
    pub err: String,
}

impl EpochError {
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

impl From<ComponentRange> for EpochError {
    fn from(err: ComponentRange) -> Self {
        Self {
            err: err.to_string(),
        }
    }
}

impl From<TryFromIntError> for EpochError {
    fn from(err: TryFromIntError) -> Self {
        Self {
            err: err.to_string(),
        }
    }
}
