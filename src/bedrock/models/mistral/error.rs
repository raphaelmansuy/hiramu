use crate::bedrock::error::BedrockError;
use serde_json::Error as SerdeJsonError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MistralError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] SerdeJsonError),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::str::Utf8Error),

    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    #[error("Unknown error: {0}")]
    Unknown(String),

    #[error("Bedrock error: {0}")]
    Bedrock(#[from] BedrockError),
}