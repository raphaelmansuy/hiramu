use aws_sdk_bedrockruntime::operation::invoke_model::InvokeModelError;
use serde_json::Error as SerdeJsonError;
use thiserror::Error;

use aws_sdk_bedrockruntime::error::SdkError;
use aws_sdk_bedrockruntime::operation::invoke_model_with_response_stream::InvokeModelWithResponseStreamError;
use aws_sdk_bedrockruntime::types::error::ResponseStreamError;
use aws_smithy_types::event_stream::RawMessage;

#[derive(Error, Debug)]
pub enum BedrockError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] SerdeJsonError),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::str::Utf8Error),

    #[error("Bedrock runtime error: {0}")]
    AwsBedrockRuntimeError(#[from] aws_sdk_bedrockruntime::Error),

    #[error("Bedrock error: {0}")]
    BedrockError(#[from] aws_sdk_bedrock::Error),

    #[error("AWS SDK error: {0}")]
    AwsSdkError(#[from] SdkError<ResponseStreamError, RawMessage>),

    #[error("AWS SDK invoke model error: {0}")]
    AwsSdkErrorInvoke(#[from] SdkError<InvokeModelWithResponseStreamError>),

    #[error("AWS SDK invoke model error: {0}")]
    AwsSdkErrorInvokeModel(#[from] SdkError<InvokeModelError>),

    #[error("Unknown error: {0}")]
    Unknown(String),
}
