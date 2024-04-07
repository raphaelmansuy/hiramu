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
#[serde(untagged)]
pub enum MessageContent {
    Text(String),
    Blocks(Vec<ContentBlock>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: MessageContent,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ContentBlock {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image")]
    Image { source: ImageSource },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageSource {
    #[serde(rename = "type")]
    pub source_type: String,
    pub media_type: String,
    pub data: String,
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

    pub fn new_user_message_with_image(text: &String, image: &String, mime_type: &String) -> Self {
        let image_block = ContentBlock::Image {
            source: ImageSource {
                source_type: "base64".to_string(),
                media_type: mime_type.to_string(),
                data: image.to_string(),
            },
        };

        let text_block = ContentBlock::Text { text: text.to_string() };


        Message {
            role: Role::User,
            content: MessageContent::Blocks(vec![text_block, image_block]),
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
    pub content: Vec<ContentBlock>,
    pub stop_reason: StopReason,
    pub stop_sequence: Option<String>,
    pub usage: UsageInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsageInfo {
    pub input_tokens: i32,
    pub output_tokens: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamResult {
    #[serde(rename = "type")]
    pub result_type: String,
    #[serde(flatten)]
    pub data: serde_json::Value,
}


#[derive(Debug, Serialize, Deserialize)]
pub enum StreamResultData {
    MessageStart(MessageStart),
    ContentBlockStart(ContentBlockStart),
    ContentBlockDelta(ContentBlockDelta),
    ContentBlockStop(ContentBlockStop),
    MessageDelta(MessageDelta),
    MessageStop(MessageStop),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageStart {
    pub message: Message,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Usage {
    pub input_tokens: i32,
    pub output_tokens: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContentBlockStart {
    pub content_block: ContentBlock,
    pub index: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContentBlockDelta {
    pub delta: Delta,
    pub index: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Delta {
    pub text: String,
    #[serde(rename = "type")]
    pub delta_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContentBlockStop {
    pub index: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageDelta {
    pub delta: MessageDeltaData,
    pub usage: MessageDeltaUsage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageDeltaData {
    pub stop_reason: String,
    pub stop_sequence: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageDeltaUsage {
    pub output_tokens: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageStop {
    #[serde(rename = "amazon-bedrock-invocationMetrics")]
    pub invocation_metrics: InvocationMetrics,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvocationMetrics {
    #[serde(rename = "firstByteLatency")]
    pub first_byte_latency: i32,
    #[serde(rename = "inputTokenCount")]
    pub input_token_count: i32,
    #[serde(rename = "invocationLatency")]
    pub invocation_latency: i32,
    #[serde(rename = "outputTokenCount")]
    pub output_token_count: i32,
}

/// --------------------------------
///  Stream Message Deserialization
/// --------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamMessage {
    message: StreamMessageContent,
    #[serde(rename = "type")]
    message_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamMessageContent {
    content: Vec<serde_json::Value>,
    id: String,
    model: String,
    role: String,
    stop_reason: Option<serde_json::Value>,
    stop_sequence: Option<serde_json::Value>,
    #[serde(rename = "type")]
    content_type: String,
    usage: StreamUsage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamUsage {
    input_tokens: u32,
    output_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamContentBlock {
    content_block: StreamContentBlockContent,
    index: u32,
    #[serde(rename = "type")]
    block_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamContentBlockContent {
    text: String,
    #[serde(rename = "type")]
    content_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamContentBlockDelta {
    delta: StreamDelta,
    index: u32,
    #[serde(rename = "type")]
    delta_type: String,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct StreamDelta {
    text: String,
    #[serde(rename = "type")]
    delta_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamMessageDelta {
    delta: StreamMessageDeltaContent,
    #[serde(rename = "type")]
    delta_type: String,
    usage: StreamMessageDeltaUsage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamMessageDeltaContent {
    stop_reason: String,
    stop_sequence: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamMessageDeltaUsage {
    output_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamInvocationMetrics {
    #[serde(rename = "amazon-bedrock-invocationMetrics")]
    metrics: StreamInvocationMetricsContent,
    #[serde(rename = "type")]
    metrics_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamInvocationMetricsContent {
    #[serde(rename = "firstByteLatency")]
    first_byte_latency: u32,
    #[serde(rename = "inputTokenCount")]
    input_token_count: u32,
    #[serde(rename = "invocationLatency")]
    invocation_latency: u32,
    #[serde(rename = "outputTokenCount")]
    output_token_count: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_stream_documents() {
        let documents = [
            r#"{"message":{"content":[],"id":"msg_01SSVH6oAf3LoGzpz3YrdxgH","model":"claude-3-haiku-48k-20240307","role":"assistant","stop_reason":null,"stop_sequence":null,"type":"message","usage":{"input_tokens":20,"output_tokens":1}},"type":"message_start"}"#,
            r#"{"content_block":{"text":"","type":"text"},"index":0,"type":"content_block_start"}"#,
            r#"{"delta":{"text":"The","type":"text_delta"},"index":0,"type":"content_block_delta"}"#,
            r#"{"delta":{"text":" capital","type":"text_delta"},"index":0,"type":"content_block_delta"}"#,
            r#"{"delta":{"text":" of","type":"text_delta"},"index":0,"type":"content_block_delta"}"#,
            r#"{"delta":{"text":" France","type":"text_delta"},"index":0,"type":"content_block_delta"}"#,
            r#"{"delta":{"text":" is","type":"text_delta"},"index":0,"type":"content_block_delta"}"#,
            r#"{"delta":{"text":" Paris","type":"text_delta"},"index":0,"type":"content_block_delta"}"#,
            r#"{"delta":{"text":".","type":"text_delta"},"index":0,"type":"content_block_delta"}"#,
            r#"{"index":0,"type":"content_block_stop"}"#,
            r#"{"delta":{"stop_reason":"end_turn","stop_sequence":null},"type":"message_delta","usage":{"output_tokens":10}}"#,
            r#"{"amazon-bedrock-invocationMetrics":{"firstByteLatency":320,"inputTokenCount":20,"invocationLatency":394,"outputTokenCount":10},"type":"message_stop"}"#,
        ];

        for document in &documents {
            match serde_json::from_str::<serde_json::Value>(document).unwrap() {
                serde_json::Value::Object(map) => {
                    match map.get("type").and_then(|v| v.as_str()) {
                        Some("message_start") => {
                            let _message: StreamMessage = serde_json::from_str(document).unwrap();
                        }
                        Some("content_block_start") => {
                            let _content_block: StreamContentBlock =
                                serde_json::from_str(document).unwrap();
                        }
                        Some("content_block_delta") => {
                            let _content_block_delta: StreamContentBlockDelta =
                                serde_json::from_str(document).unwrap();
                        }
                        Some("content_block_stop") => {
                            // No specific struct for content_block_stop
                        }
                        Some("message_delta") => {
                            let _message_delta: StreamMessageDelta =
                                serde_json::from_str(document).unwrap();
                        }
                        Some("message_stop") => {
                            let _invocation_metrics: StreamInvocationMetrics =
                                serde_json::from_str(document).unwrap();
                        }
                        _ => panic!("Unknown document type"),
                    }
                }
                _ => panic!("Invalid JSON document"),
            }
        }
    }
}