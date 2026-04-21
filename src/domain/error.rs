use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum DomainError {
    #[error("Not authenticated")]
    NotAuthenticated,

    #[error("API error (code {0}): {1}")]
    Api(i32, String),

    #[error("Config error: {0}")]
    Config(String),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("IO error: {0}")]
    Io(String),

    #[error("HTTP error: {0}")]
    Http(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

// Implement From for common error types
impl From<reqwest::Error> for DomainError {
    fn from(err: reqwest::Error) -> Self {
        DomainError::Http(err.to_string())
    }
}

impl From<std::io::Error> for DomainError {
    fn from(err: std::io::Error) -> Self {
        DomainError::Io(err.to_string())
    }
}

impl From<serde_json::Error> for DomainError {
    fn from(err: serde_json::Error) -> Self {
        DomainError::Parse(err.to_string())
    }
}

pub type DomainResult<T> = std::result::Result<T, DomainError>;
