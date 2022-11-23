use thiserror::Error;

pub type Result<T> = std::result::Result<T, CollectorError>;

#[derive(Error, Debug)]
pub enum CollectorError {
    /// IO related errors
    #[error("{0}")]
    Io(#[from] std::io::Error),
    /// general errors we don't know where to put
    #[error("{0}")]
    General(String),
    /// deserialization errors
    #[error("{0}")]
    Format(String),
}