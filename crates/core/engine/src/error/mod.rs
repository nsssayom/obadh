//! Error types and handling

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("System error: {0}")]
    SystemError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
