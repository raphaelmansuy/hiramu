use serde::{Deserialize, Serialize};

/// Represents a Request Message to be sent to the Claude API.
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestMessage {
    /// The Anthropic version. Must be "bedrock-2023-05-31".
    pub anthropic_version: String,
    /// The maximum number of tokens to generate before stopping.
    pub max_tokens: u32,
    /// The input messages for the conversation.
    pub messages: Vec<Message>,
    /// The optional system prompt to provide context and instructions to Claude.
    pub system: Option<String>,
    /// Custom text sequences that cause the model to stop generating.
    pub stop_sequences: Option<Vec<String>>,
    /// The amount of randomness injected into the response. Range: 0.0 to 1.0.
    pub temperature: Option<f32>,
    /// Nucleus sampling parameter. Range: 0.0 to 1.0.
    pub top_p: Option<f32>,
    /// Sample from the top K options for each subsequent token.
    pub top_k: Option<u32>,
}

/// Represents a message in the conversation.
#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    /// The role of the message sender. Valid values: "user" or "assistant".
    pub role: String,
    /// The content of the message.
    pub content: Content,
}

/// Represents the content of a message, which can be either text or blocks.
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Content {
    /// Text content.
    Text(String),
    /// Content blocks.
    Blocks(Vec<ContentBlock>),
}

/// Represents a content block within a message.
#[derive(Serialize, Deserialize, Debug)]
pub struct ContentBlock {
    /// The type of the content block. Valid values: "text" or "image".
    #[serde(rename = "type")]
    pub block_type: String,
    /// The data associated with the content block.
    #[serde(flatten)]
    pub data: ContentBlockData,
}

/// Represents the data associated with a content block.
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ContentBlockData {
    /// Text data.
    Text { text: String },
    /// Image data.
    Image { source: ImageSource },
}

/// Represents the source of an image.
#[derive(Serialize, Deserialize, Debug)]
pub struct ImageSource {
    /// The encoding type for the image. Must be "base64".
    #[serde(rename = "type")]
    pub source_type: String,
    /// The media type of the image. Valid values: "image/jpeg", "image/png", "image/webp", "image/gif".
    pub media_type: String,
    /// The base64-encoded image data.
    pub data: String,
}

impl RequestMessage {
    /// Creates a new `RequestMessage` instance with the specified `max_tokens` and `messages`.
    pub fn new(max_tokens: u32, messages: Vec<Message>) -> Self {
        RequestMessage {
            anthropic_version: "bedrock-2023-05-31".to_string(),
            max_tokens,
            messages,
            system: None,
            stop_sequences: None,
            temperature: None,
            top_p: None,
            top_k: None,
        }
    }

    /// Sets the system prompt for the `RequestMessage`.
    pub fn with_system(mut self, system: String) -> Self {
        self.system = Some(system);
        self
    }

    /// Sets the stop sequences for the `RequestMessage`.
    pub fn with_stop_sequences(mut self, stop_sequences: Vec<String>) -> Self {
        self.stop_sequences = Some(stop_sequences);
        self
    }

    /// Sets the temperature for the `RequestMessage`.
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    /// Sets the top_p value for the `RequestMessage`.
    pub fn with_top_p(mut self, top_p: f32) -> Self {
        self.top_p = Some(top_p);
        self
    }

    /// Sets the top_k value for the `RequestMessage`.
    pub fn with_top_k(mut self, top_k: u32) -> Self {
        self.top_k = Some(top_k);
        self
    }
}

impl Message {
    /// Creates a new user message with the specified content.
    pub fn new_user_message(content: impl Into<Content>) -> Self {
        Message {
            role: "user".to_string(),
            content: content.into(),
        }
    }

    /// Creates a new assistant message with the specified content.
    pub fn new_assistant_message(content: impl Into<Content>) -> Self {
        Message {
            role: "assistant".to_string(),
            content: content.into(),
        }
    }
}

impl From<String> for Content {
    fn from(text: String) -> Self {
        Content::Text(text)
    }
}

impl From<Vec<ContentBlock>> for Content {
    fn from(blocks: Vec<ContentBlock>) -> Self {
        Content::Blocks(blocks)
    }
}

impl ContentBlock {
    /// Creates a new text content block with the specified text.
    pub fn new_text_block(text: String) -> Self {
        ContentBlock {
            block_type: "text".to_string(),
            data: ContentBlockData::Text { text },
        }
    }

    /// Creates a new image content block with the specified media type and base64-encoded data.
    pub fn new_image_block(media_type: String, data: String) -> Self {
        ContentBlock {
            block_type: "image".to_string(),
            data: ContentBlockData::Image {
                source: ImageSource {
                    source_type: "base64".to_string(),
                    media_type,
                    data,
                },
            },
        }
    }
}