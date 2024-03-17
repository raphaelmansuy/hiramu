use serde_json::Error as SerdeJsonError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HiramuError {
    #[error("HTTP error: {0}")] Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")] Json(#[from] SerdeJsonError),
}
