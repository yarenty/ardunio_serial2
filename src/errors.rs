use std::string::FromUtf8Error;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, CollectorError>;

#[derive(Error, Debug)]
pub enum CollectorError {
    /// IO related errors
    #[error("{0}")]
    Io(#[from] std::io::Error),
    /// Conversion errors
    #[error("{0}")]
    Conversion(#[from] FromUtf8Error),
    /// Unknown errors
    #[error("{0}")]
    Unknown(String),
}
