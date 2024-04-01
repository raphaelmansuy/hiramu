use crate::bedrock::bedrock_client::BedrockClient;
use crate::bedrock::models::claude::claude_error::ClaudeError;
use crate::bedrock::models::claude::claude_request::ClaudeRequest;
use crate::bedrock::models::claude::claude_request_message::{
     Message, RequestMessage,
};
use crate::bedrock::models::claude::claude_response::ClaudeResponse;
use serde_json::Value;
use crate::bedrock::models::claude::claude_response_message::{ResponseMessage, StopReason, Usage,ContentBlock as ChatContentBlock};

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

pub struct ChatResponse {
    pub messages: Vec<Message>,
    // Add other relevant fields
}

// Add the missing import statement for ResponseMessage

fn parse_chat_response(response: Value) -> Result<ResponseMessage, ClaudeError> {
    // Extract the relevant fields from the response JSON
    let id = response["id"]
        .as_str()
        .ok_or_else(|| ClaudeError::InvalidResponse("Missing id field".to_string()))?
        .to_string();

    let model = response["model"]
        .as_str()
        .ok_or_else(|| ClaudeError::InvalidResponse("Missing model field".to_string()))?
        .to_string();

    let stop_reason = response["stop_reason"]
        .as_str()
        .ok_or_else(|| ClaudeError::InvalidResponse("Missing stop_reason field".to_string()))?;

    let stop_reason = match stop_reason {
        "end_turn" => StopReason::EndTurn,
        "max_tokens" => StopReason::MaxTokens,
        "stop_sequence" => StopReason::StopSequence,
        _ => {
            return Err(ClaudeError::InvalidResponse(
                "Invalid stop_reason value".to_string(),
            ))
        }
    };

    let response_type = response["type"]
        .as_str()
        .ok_or_else(|| ClaudeError::InvalidResponse("Missing type field".to_string()))?;

    if response_type != "message" {
        return Err(ClaudeError::InvalidResponse(
            "Invalid type value".to_string(),
        ));
    }

    let role = response["role"]
        .as_str()
        .ok_or_else(|| ClaudeError::InvalidResponse("Missing role field".to_string()))?;

    if role != "assistant" {
        return Err(ClaudeError::InvalidResponse(
            "Invalid role value".to_string(),
        ));
    }

    let content = response["content"]
        .as_array()
        .ok_or_else(|| ClaudeError::InvalidResponse("Missing content field".to_string()))?;

    let content_blocks: Result<Vec<ChatContentBlock>, ClaudeError> = content
        .iter()
        .map(|block| {
            let block_type = block["type"].as_str().ok_or_else(|| {
                ClaudeError::InvalidResponse("Missing content block type".to_string())
            })?;

            if block_type != "text" {
                return Err(ClaudeError::InvalidResponse(
                    "Invalid content block type".to_string(),
                ));
            }

            let text = block["text"]
                .as_str()
                .ok_or_else(|| {
                    ClaudeError::InvalidResponse("Missing content block text".to_string())
                })?
                .to_string();

            Ok(ChatContentBlock {
                r#type: block_type.to_string(),
                text,
            })
        })
        .collect();

    let content_blocks = content_blocks?;

    let input_tokens = response["usage"]["input_tokens"]
        .as_u64()
        .ok_or_else(|| ClaudeError::InvalidResponse("Missing input_tokens field".to_string()))?;

    let output_tokens = response["usage"]["output_tokens"]
        .as_u64()
        .ok_or_else(|| ClaudeError::InvalidResponse("Missing output_tokens field".to_string()))?;

    let stop_sequence = response["stop_sequence"].as_str().map(|s| s.to_string());

    let response_message = ResponseMessage {
        id,
        model,
        stop_reason,
        r#type: "message".to_string(),
        role: "assistant".to_string(),
        content: content_blocks,
        usage: Usage {
            input_tokens: input_tokens as u32,
            output_tokens: output_tokens as u32,
        },
        stop_sequence,
    };

    Ok(response_message)
}


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
                let completion = parse_completion_response(response)?;
                Ok(completion)
            }
            Err(err) => Err(ClaudeError::Unknown(err.to_string())),
        }
    }

    pub async fn chat(
        &self,
        messages: &[Message],
        options: ChatOptions,
    ) -> Result<ResponseMessage, ClaudeError> {

        // Rest of the code...

        let request = RequestMessage::new(options.max_tokens, messages.to_vec())
            .with_temperature(options.temperature.unwrap_or(0.0))
            .with_top_p(options.top_p.unwrap_or(1.0))
            .with_top_k(options.top_k.unwrap_or(50))
            .with_stop_sequences(options.stop_sequences.unwrap_or(vec![]));

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
                let chat_response = parse_chat_response(response)?;
                Ok(chat_response)
            }
            Err(err) => Err(ClaudeError::Unknown(err.to_string())),
        }
    
        }
}

fn parse_completion_response(response: Value) -> Result<ClaudeResponse, ClaudeError> {
    // Extract the relevant fields from the response JSON
    let completion = response["completion"]
        .as_str()
        .ok_or_else(|| ClaudeError::InvalidResponse("Missing completion field".to_string()))?
        .to_string();

    let stop_reason = response["stop_reason"]
        .as_str()
        .ok_or_else(|| ClaudeError::InvalidResponse("Missing stop_reason field".to_string()))?
        .to_string();

    let stop = response["stop"]
        .as_str()
        .ok_or_else(|| ClaudeError::InvalidResponse("Missing stop field".to_string()))?
        .to_string();

    // Create a new CauseResponse instance with the extracted fields
    let claude_response = ClaudeResponse::new(&completion, &stop_reason, &stop);

    Ok(claude_response)
}
