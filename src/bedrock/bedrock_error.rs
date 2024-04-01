use thiserror::Error;

#[derive(Error, Debug)]
pub enum BedrockError {
    #[error("Error from serde_json: {0}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("Error from aws_sdk_bedrockruntime: {0}")]
    SdkError(#[from] aws_sdk_bedrockruntime::Error),

    #[error("Unknown error: {0}")]
    Unknown(String),
}