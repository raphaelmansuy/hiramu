use thiserror::Error;
use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeJsonError;
use std::str::Utf8Error;

#[derive(Error, Debug)]
pub enum GenerateError {
    #[error("HTTP error: {0}")]
    Http(#[from] ReqwestError),
    #[error("JSON error: {0}")]
    Json(#[from] SerdeJsonError),
    #[error("UTF-8 error: {0}")]
    Utf8(#[from] Utf8Error),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Unknown error: {0}")]
    Unknown(String),
}