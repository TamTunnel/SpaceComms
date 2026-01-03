//! SpaceComms error types

use thiserror::Error;

/// SpaceComms result type
pub type Result<T> = std::result::Result<T, Error>;

/// SpaceComms error types
#[derive(Error, Debug)]
pub enum Error {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("YAML parsing error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("CDM validation error: {0}")]
    CdmValidation(String),

    #[error("Protocol error: {0}")]
    Protocol(String),

    #[error("Peer error: {0}")]
    Peer(String),

    #[error("Storage error: {0}")]
    Storage(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Already exists: {0}")]
    AlreadyExists(String),

    #[error("HTTP client error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl Error {
    /// Returns true if this is a not found error
    pub fn is_not_found(&self) -> bool {
        matches!(self, Error::NotFound(_))
    }

    /// Returns true if this is a validation error
    pub fn is_validation(&self) -> bool {
        matches!(self, Error::CdmValidation(_))
    }
}
