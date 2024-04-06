use crate::bedrock::bedrock_client::{BedrockClient, BedrockClientOptions};
use crate::bedrock::models::claude::claude_request_message::{
    ChatOptions, ConversationRequest, ConversationResponse, StreamResult,
};
use crate::bedrock::models::claude::error::ClaudeError;
use futures::stream::Stream;
use futures::TryStreamExt;
use serde_json::Value;

use super::claude_request_message::{
    ContentBlockDelta, ContentBlockStart, ContentBlockStop, MessageDelta, MessageStart,
    MessageStop, StreamResultData,
};

pub type ClaudeOptions = BedrockClientOptions;

pub struct ClaudeClient {
    client: BedrockClient,
}

impl ClaudeClient {
    /// Constructs a new `BedrockClient`.
    pub async fn new(options: ClaudeOptions) -> Self {
        Self {
            client: BedrockClient::new(options).await,
        }
    }

    pub async fn chat(
        &self,
        request: &ConversationRequest,
        options: &ChatOptions,
    ) -> Result<ConversationResponse, ClaudeError> {
        let model_id = options.model_id.to_string();
        let payload = serde_json::to_value(request);

        let payload = match payload {
            Ok(payload) => payload,
            Err(err) => return Err(ClaudeError::Json(err)),
        };

        let response = self.client.generate_raw(model_id, payload).await;
        match response {
            Ok(response) => {
                let conversation_response = serde_json::from_value(response).unwrap();
                Ok(conversation_response)
            }
            Err(err) => Err(ClaudeError::from(err)),
        }
    }

    pub async fn chat_with_stream(
        &self,
        request: &ConversationRequest,
        options: &ChatOptions,
    ) -> Result<impl Stream<Item = Result<StreamResultData, ClaudeError>>, ClaudeError> {
        let model_id = options.model_id.to_string();
        let payload = serde_json::to_value(request).map_err(|err| ClaudeError::Json(err))?;

        let response = self.client.generate_raw_stream(model_id, payload).await?;

        let stream = response
            .map_err(ClaudeError::from)
            .and_then(|chunk| async move {
                let stream_result = deserialize_stream_result(chunk)?;
                Ok(stream_result)
            });

        Ok(stream)
    }
}

fn deserialize_stream_result(value: Value) -> Result<StreamResultData, ClaudeError> {
    let stream_result: StreamResult = serde_json::from_value(value)
        .map_err(|err| ClaudeError::Deserialization(err.to_string()))?;

    match stream_result.result_type.as_str() {
        "message_start" => {
            let message_start: MessageStart = serde_json::from_value(stream_result.data)
                .map_err(|err| ClaudeError::Deserialization(err.to_string()))?;
            Ok(StreamResultData::MessageStart(message_start))
        }
        "content_block_start" => {
            let content_block_start: ContentBlockStart = serde_json::from_value(stream_result.data)
                .map_err(|err| ClaudeError::Deserialization(err.to_string()))?;
            Ok(StreamResultData::ContentBlockStart(content_block_start))
        }
        "content_block_delta" => {
            let content_block_delta: ContentBlockDelta = serde_json::from_value(stream_result.data)
                .map_err(|err| ClaudeError::Deserialization(err.to_string()))?;
            Ok(StreamResultData::ContentBlockDelta(content_block_delta))
        }
        "content_block_stop" => {
            let content_block_stop: ContentBlockStop =
                serde_json::from_value(stream_result.data)
                    .map_err(|err| ClaudeError::Deserialization(err.to_string()))?;
            Ok(StreamResultData::ContentBlockStop(content_block_stop))
        }
        "message_delta" => {
            let message_delta: MessageDelta = serde_json::from_value(stream_result.data)
                .map_err(|err| ClaudeError::Deserialization(err.to_string()))?;
            Ok(StreamResultData::MessageDelta(message_delta))
        }
        "message_stop" => {
            let message_stop: MessageStop = serde_json::from_value(stream_result.data)
                .map_err(|err| ClaudeError::Deserialization(err.to_string()))?;
            Ok(StreamResultData::MessageStop(message_stop))
        }
        _ => Err(ClaudeError::Deserialization(format!(
            "Unknown StreamResult type: {}",
            stream_result.result_type
        ))),
    }
}
