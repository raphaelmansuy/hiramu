use serde_json::Error as SerdeJsonError;
use thiserror::Error;

/// Represents all possible errors that can occur in the application.
///
/// This enum is used throughout the application to represent any error that can occur.
/// It includes variants for HTTP errors, JSON parsing errors, I/O errors, UTF-8 conversion errors,
/// invalid responses, API errors, and a catch-all for unknown errors.
///
/// Each variant contains an error message that provides more details about the error.
///
/// # Variants
///
/// * `Http` - Represents an error that occurred while making an HTTP request.
/// * `Json` - Represents an error that occurred while parsing JSON.
/// * `Io` - Represents an error that occurred while performing an I/O operation.
/// * `Utf8` - Represents an error that occurred while converting a string to UTF-8.
/// * `InvalidResponse` - Represents an error that occurred due to an invalid response from an API.
/// * `ApiError` - Represents an error that occurred while interacting with an API.
/// * `Unknown` - Represents an unknown error.
///
/// Each variant can be created from the corresponding error type using the `From` trait.
#[derive(Error, Debug)]
pub enum HiramuError {
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

    #[error("API error: {0}")]
    ApiError(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}