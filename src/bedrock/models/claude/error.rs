use serde_json::Error as SerdeJsonError;
use thiserror::Error;

use crate::bedrock::error::BedrockError;


#[derive(Error, Debug)]
pub enum ClaudeError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] SerdeJsonError),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::str::Utf8Error),
    
    #[error("Bedrock error: {0}")]
    Aws(#[from] BedrockError),

    #[error("Unknown error: {0}")]
    Unknown(String),

    #[error("Deserialization error: {0}")]
    Deserialization(String),
}
