use crate::bedrock::bedrock_client::BedrockClient;
use crate::bedrock::models::claude::claude_error::ClaudeError;
use crate::bedrock::models::claude::claude_request_message::{
    ChatOptions, ConversationRequest, ConversationResponse, StreamResult,
};
use futures::stream::Stream;
use futures::TryStreamExt;

pub struct ClaudeClient {
    bedrock_client: BedrockClient,
    region: String,
    profile_name: String,
}

impl ClaudeClient {
    pub fn new(profile_name: String, region: String) -> Self {
        let bedrock_client = BedrockClient::new();
        Self {
            bedrock_client,
            region,
            profile_name,
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

        let response = self
            .bedrock_client
            .generate_raw(
                model_id,
                payload,
                Some(self.profile_name.clone()),
                Some(self.region.clone()),
            )
            .await;
        match response {
            Ok(response) => {
                let conversation_response = serde_json::from_value(response).unwrap();
                Ok(conversation_response)
            }
            Err(err) => Err(ClaudeError::Unknown(err.to_string())),
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

        let response = self
            .bedrock_client
            .generate_raw_stream(
                model_id,
                payload,
                Some(self.profile_name.clone()),
                Some(self.region.clone()),
            )
            .await;

        let response = match response {
            Ok(response) => response,
            Err(err) => return Err(ClaudeError::ApiError(err.to_string())),
        };

        Ok(response
            .map_ok(|value| {
                serde_json::from_value(value).map_err(ClaudeError::Json)
            })
            .map_err(|err| ClaudeError::Unknown(err.to_string()))
            .and_then(futures::future::ready))
    }
}