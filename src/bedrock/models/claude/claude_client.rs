use serde_json::Value;

use crate::bedrock::bedrock_client::BedrockClient;
use crate::bedrock::models::claude::claude_error::ClaudeError;
use crate::bedrock::models::claude::claude_request_message::{
    ChatOptions, ConversationRequest, ConversationResponse,
};

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
        options: ChatOptions,
    ) -> Result<ConversationResponse, ClaudeError> {
        let model_id = options.model_id.to_string();
        let payload: Value = serde_json::to_value(request).unwrap();
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
                //display the response, JSON formatted
                let conversaton_response = serde_json::from_value(response).unwrap();
                Ok(conversaton_response)
            }
            Err(err) => Err(ClaudeError::Unknown(err.to_string())),
        }
    }
}
