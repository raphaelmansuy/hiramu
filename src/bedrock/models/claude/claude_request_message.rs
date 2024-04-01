use serde::{Deserialize, Serialize};

pub struct ChatOptions {
    pub model_id: String,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub top_k: Option<u32>,
    pub max_tokens: u32,
    pub stop_sequences: Option<Vec<String>>,
}

impl Default for ChatOptions {
    fn default() -> Self {
        ChatOptions {
            model_id: "anthropic.claude-3-haiku-20240307-v1:0".to_string(),
            temperature: Some(0.5),
            top_p: Some(1.0),
            top_k: Some(50),
            max_tokens: 100,
            stop_sequences: Some(vec![]),
        }
    }
}

impl ChatOptions {
    pub fn with_model_id(mut self, model_id: String) -> Self {
        self.model_id = model_id;
        self
    }

    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    pub fn with_top_p(mut self, top_p: f32) -> Self {
        self.top_p = Some(top_p);
        self
    }

    pub fn with_top_k(mut self, top_k: u32) -> Self {
        self.top_k = Some(top_k);
        self
    }

    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = max_tokens;
        self
    }

    pub fn with_stop_sequences(mut self, stop_sequences: Vec<String>) -> Self {
        self.stop_sequences = Some(stop_sequences);
        self
    }

    pub fn add_stop_sequence(mut self, stop_sequence: String) -> Self {
        match &mut self.stop_sequences {
            Some(sequences) => sequences.push(stop_sequence),
            None => self.stop_sequences = Some(vec![stop_sequence]),
        }
        self
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: MessageContent,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageContent {
    Text(String),
    Blocks(Vec<ContentBlock>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContentBlock {
    #[serde(rename = "type")]
    pub block_type: String,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConversationRequest {
    pub system: Option<String>,
    pub messages: Vec<Message>,
    pub max_tokens: Option<i32>,
    pub anthropic_version: String,
}

impl Default for ConversationRequest {
    fn default() -> Self {
        ConversationRequest {
            system: Some("Your are a useful assistant.".to_string()),
            messages: Vec::new(),
            max_tokens: Some(1024),
            anthropic_version: "bedrock-2023-05-31".to_string(),
        }
    }
}

impl Message {
    pub fn new_user_message(content: impl Into<MessageContent>) -> Self {
        Message {
            role: Role::User,
            content: content.into(),
        }
    }

    pub fn new_assistant_message(content: impl Into<MessageContent>) -> Self {
        Message {
            role: Role::Assistant,
            content: content.into(),
        }
    }
}

impl From<String> for MessageContent {
    fn from(text: String) -> Self {
        MessageContent::Text(text)
    }
}

impl From<Vec<ContentBlock>> for MessageContent {
    fn from(blocks: Vec<ContentBlock>) -> Self {
        MessageContent::Blocks(blocks)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StopReason {
    #[serde(rename = "end_turn")]
    EndTurn,
    #[serde(rename = "max_tokens")]
    MaxTokens,
    #[serde(rename = "stop_sequence")]
    StopSequence,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConversationResponse {
    pub id: String,
    pub model: String,
    #[serde(rename = "type")]
    pub response_type: String,
    pub role: Role,
    pub content: Vec<ResponseContentBlock>,
    pub stop_reason: StopReason,
    pub stop_sequence: Option<String>,
    pub usage: UsageInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseContentBlock {
    #[serde(rename = "type")]
    pub block_type: String,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsageInfo {
    pub input_tokens: i32,
    pub output_tokens: i32,
}
