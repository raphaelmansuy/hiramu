use crate::bedrock::bedrock_client::{BedrockClient, BedrockClientOptions};
use crate::bedrock::models::claude::claude_request_message::{
    ChatOptions, ConversationRequest, ConversationResponse, StreamResult,
};
use crate::bedrock::models::claude::error::ClaudeError;
use futures::stream::Stream;
use futures::TryStreamExt;

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
    ) -> Result<impl Stream<Item = Result<StreamResult, ClaudeError>>, ClaudeError> {
        let model_id = options.model_id.to_string();
        let payload = serde_json::to_value(request);

        let payload = match payload {
            Ok(payload) => payload,
            Err(err) => return Err(ClaudeError::Json(err)),
        };

        let response = self.client.generate_raw_stream(model_id, payload).await;

        let response = match response {
            Ok(response) => response,
            Err(err) => return Err(ClaudeError::from(err)),
        };

        Ok(response
            .map_ok(|value| serde_json::from_value(value).map_err(ClaudeError::Json))
            .map_err(|err| ClaudeError::Unknown(err.to_string()))
            .and_then(futures::future::ready))
    }
}
