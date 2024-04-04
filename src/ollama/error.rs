use serde_json::Error as SerdeJsonError;
use thiserror::Error;



#[derive(Error, Debug)]
pub enum OllamaError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] SerdeJsonError),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Utf8 error: {0}")]
    Utf8(#[from] std::str::Utf8Error),

    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    #[error("Bad Request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Not Found: {0}")]
    NotFound(String),

    #[error("Too Many Requests: {0}")]
    TooManyRequests(String),

    #[error("Internal Server Error: {0}")]
    InternalServerError(String),

    #[error("Unknown API Error: {0}")]
    UnknownApiError(String),

    #[error("Request Builder Error: {0}")]
    RequestBuilderError(String),

    #[error("Deserialization Error: {0}")]
    DeserializationError(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}