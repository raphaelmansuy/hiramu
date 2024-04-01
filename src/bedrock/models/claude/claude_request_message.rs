use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
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


impl Message {
    pub fn new_user_message_content(content: impl Into<MessageContent>) -> Self {
        Message {
            role: "user".to_string(),
            content: content.into(),
        }
    }

    pub fn new_assistant_message_content(content: impl Into<MessageContent>) -> Self {
        Message {
            role: "assistant".to_string(),
            content: content.into(),
        }
    }

    pub fn new_user_message(content: String) -> Self {
        Message {
            role: "user".to_string(),
            content: MessageContent::Text(content.clone()),
        }
    }

    pub fn new_assistant_message(content: String) -> Self {
        Message {
            role: "assistant".to_string(),
            content: MessageContent::Text(content.clone()),
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
pub struct ConversationResponse {
    pub id: String,
    pub model: String,
    #[serde(rename = "type")]
    pub response_type: String,
    pub role: String,
    pub content: Vec<ResponseContentBlock>,
    pub stop_reason: String,
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