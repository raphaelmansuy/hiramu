use serde_json::Value;

use crate::bedrock::bedrock_client::BedrockClient;
use crate::bedrock::models::claude::claude_error::ClaudeError;
use crate::bedrock::models::claude::claude_request::ClaudeRequest;
use crate::bedrock::models::claude::claude_response::ClaudeResponse;
use crate::bedrock::models::claude::claude_request_message::{ConversationRequest,ConversationResponse};

pub struct CompletionOptions {
    pub model_id: String,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub top_k: Option<usize>,
    pub max_tokens: usize,
    pub stop_sequences: Option<Vec<String>>,
}

pub struct ChatOptions {
    pub model_id: String,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub top_k: Option<u32>,
    pub max_tokens: u32,
    pub stop_sequences: Option<Vec<String>>,
}

// Add the missing import statement for ResponseMessage

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

    pub async fn complete(
        &self,
        prompt: &str,
        options: CompletionOptions,
    ) -> Result<ClaudeResponse, ClaudeError> {
        let request = ClaudeRequest::new(prompt)
            .with_temperature(options.temperature.unwrap_or(0.0))
            .with_top_p(options.top_p.unwrap_or(0.0))
            .with_top_k(options.top_k.unwrap_or(0))
            .with_max_tokens(options.max_tokens)
            .with_stop_sequences(
                options
                    .stop_sequences
                    .unwrap_or_default()
                    .iter()
                    .map(|s| s.as_str())
                    .collect(),
            );

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
                let completion = serde_json::from_value(response).unwrap();
                Ok(completion)
            }
            Err(err) => Err(ClaudeError::Unknown(err.to_string())),
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
                println!("{}", serde_json::to_string_pretty(&response).unwrap());
                let chat_response = serde_json::from_value(response).unwrap();
                Ok(chat_response)
            }
            Err(err) => Err(ClaudeError::Unknown(err.to_string())),
        }
    
        }
}
