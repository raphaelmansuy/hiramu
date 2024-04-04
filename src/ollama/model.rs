//! This module defines the data structures and builders for interacting with the Ollama API.
//!
//! The main components of this module are:
//! - `GenerateRequest` and `GenerateRequestBuilder`: Represents a request to generate text using the Ollama API.
//! - `GenerateResponse`: Represents a response from the Ollama API for a generate request.
//! - `ChatRequest` and `ChatRequestBuilder`: Represents a request to initiate a chat session with the Ollama API.
//! - `ChatResponse`: Represents a response from the Ollama API for a chat request.
//! - `Message`: Represents a message in a chat session, containing the role and content.
//!
//! The module provides a convenient way to construct requests using the builder pattern and
//! deserialize responses from the Ollama API.
//!
//! Example usage:
//!
//! ```
//! use ollama::model::{GenerateRequestBuilder, ChatRequestBuilder, Message};
//!
//! // Create a generate request
//! let generate_request = GenerateRequestBuilder::new("model_id".to_string())
//!     .prompt("Hello, how are you?".to_string())
//!     .build();
//!
//! // Create a chat request
//! let chat_request = ChatRequestBuilder::new("model_id".to_string())
//!     .messages(vec![
//!         Message {
//!             role: "user".to_string(),
//!             content: "Hello, how are you?".to_string(),
//!             images: vec![],
//!         },
//!     ])
//!     .build();
//! ```
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use serde_json::Value;
use pin_project::pin_project;

use super::OllamaError;

/// Represents a request to generate text using the Ollama API.
#[derive(Debug, Serialize, Clone)]
pub struct GenerateRequest {
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub images: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Vec<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_alive: Option<String>,
}
/// Represents a response from the Ollama API for a generate request.
#[pin_project]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GenerateResponse {
    pub model: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub response: String,
    pub done: bool,
    pub context: Option<Vec<u32>>,
    pub total_duration: Option<u128>,
    pub load_duration: Option<u128>,
    pub prompt_eval_count: Option<u32>,
    pub prompt_eval_duration: Option<u128>,
    pub eval_count: Option<u32>,
    pub eval_duration: Option<u128>,
}

// Implement the TryFrom trait to convert a JSON string into a GenerateResponse.
impl TryFrom<&str> for GenerateResponse {
    type Error = OllamaError;

    fn try_from(json: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(json).map_err(|e| {
            OllamaError::DeserializationError(format!(
                "Failed to deserialize GenerateResponse: {}",
                e
            ))
        })
    }
}

// Represents a builder for constructing a GenerateRequest.
pub struct GenerateRequestBuilder {
    model: String,
    prompt: Option<String>,
    images: Option<Vec<String>>,
    format: Option<String>,
    options: Option<Value>,
    system: Option<String>,
    template: Option<String>,
    context: Option<Vec<u32>>,
    stream: Option<bool>,
    raw: Option<bool>,
    keep_alive: Option<String>,
}

impl GenerateRequestBuilder {
    // Create a new GenerateRequestBuilder with the specified model.
    // The model is a required field and must be provided.
    pub fn new(model: String) -> Self {
        Self {
            model,
            prompt: None,
            images: None,
            format: None,
            options: None,
            system: None,
            template: None,
            context: None,
            stream: None,
            raw: None,
            keep_alive: None,
        }
    }

    // Set the prompt field of the GenerateRequestBuilder.
    // This field is used to provide a prompt for the generation process.
    // The value should be a string representing the prompt.
    // If the value is not provided, the prompt will be empty.
    pub fn prompt(mut self, prompt: String) -> Self {
        self.prompt = Some(prompt);
        self
    }
    

    // Set the images field of the GenerateRequestBuilder.
    // This field is used to provide a list of images for the generation process.
    // The value should be a vector of strings representing the images.
    // The images should be base64 encoded strings.
    // If the value is not provided, the images will be empty.
    pub fn images(mut self, images: Vec<String>) -> Self {
        self.images = Some(images);
        self
    }

    // Set the format field of the GenerateRequestBuilder.
    // This field is used to specify the format of the response.
    // The value should be a string representing the format.
    // If the value is not provided, the response will be returned as a string.
    pub fn format(mut self, format: String) -> Self {
        self.format = Some(format);
        self
    }

    // Set the options field of the GenerateRequestBuilder.
    // This field is used to provide options for the generation process.
    // The value should be a JSON object representing the options.
    pub fn options(mut self, options: Value) -> Self {
        self.options = Some(options);
        self
    }

    // Set the system field of the GenerateRequestBuilder.
    // This field is used to provide a system prompt for the generation process.
    // The value should be a string representing the system prompt.
    // If the value is not provided, the system prompt will be empty.
    pub fn system(mut self, system: String) -> Self {
        self.system = Some(system);
        self
    }

    // Set the template field of the GenerateRequestBuilder.
    // This field is used to provide a template for the generation process.
    // The value should be a string representing the template.
    // If the value is not provided, the template will be empty.
    pub fn template(mut self, template: String) -> Self {
        self.template = Some(template);
        self
    }

    // Set the context field of the GenerateRequestBuilder.
    // This field is used to provide a context for the generation process.
    // The value should be a vector of integers representing the context.
    // If the value is not provided, the context will be empty.
    pub fn context(mut self, context: Vec<u32>) -> Self {
        self.context = Some(context);
        self
    }


    // Set the stream field of the GenerateRequestBuilder.
    // This field is used to return a stream of responses from the API.
    // If the value is not provided, the response will be processed and returned as a string.
    pub fn stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
        self
    }

    // Set the raw field of the GenerateRequestBuilder.
    // This field is used to return the raw response from the API without any additional processing.
    // If the value is not provided, the response will be processed and returned as a string.
    pub fn raw(mut self, raw: bool) -> Self {
        self.raw = Some(raw);
        self
    }

    // Set the keep_alive field of the GenerateRequestBuilder.
    // This field is used to keep the connection alive for a specified duration.
    // The value should be a string representing the duration in seconds.
    // If the value is not provided, the connection will be closed after the response is received.
    pub fn keep_alive(mut self, keep_alive: String) -> Self {
        self.keep_alive = Some(keep_alive);
        self
    }

    // Build the GenerateRequest struct from the builder.
    pub fn build(self) -> GenerateRequest {
        GenerateRequest {
            model: self.model,
            prompt: self.prompt,
            images: self.images.unwrap_or_default(),
            format: self.format,
            options: self.options,
            system: self.system,
            template: self.template,
            context: self.context,
            raw: self.raw, 
            stream: self.stream,
            keep_alive: self.keep_alive,
        }
    }
}

// Implement the From trait to convert a GenerateRequestBuilder into a JSON string.
impl From<GenerateRequestBuilder> for String {
    fn from(request: GenerateRequestBuilder) -> Self {
        serde_json::to_string(&request.build()).unwrap()
    }
}
/// Represents a request to initiate a chat session with the Ollama API.
#[derive(Debug, Serialize, Clone)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_alive: Option<String>,
}

// Represents a message in a chat session, containing the role and content.
#[derive(Debug, Serialize,Deserialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub images: Vec<String>,
}

/// Represents a response from the Ollama API for a chat request.
#[pin_project]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatResponse {
    pub model: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub message: Message,
    pub done: bool,
    pub total_duration: Option<u128>,
    pub load_duration: Option<u128>,
    pub prompt_eval_count: Option<u32>,
    pub prompt_eval_duration: Option<u128>,
    pub eval_count: Option<u32>,
    pub eval_duration: Option<u128>,
}

impl TryFrom<&str> for ChatResponse {
    type Error = OllamaError;

    fn try_from(json: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(json).map_err(|e| {
            OllamaError::DeserializationError(format!(
                "Failed to deserialize ChatResponse: {}",
                e
            ))
        })
    }
}

pub struct ChatRequestBuilder {
    model: String,
    messages: Vec<Message>,
    format: Option<String>,
    options: Option<Value>,
    template: Option<String>,
    stream: Option<bool>,
    keep_alive: Option<String>,
}

impl ChatRequestBuilder {
    pub fn new(model: String) -> Self {
        Self {
            model,
            messages: Vec::new(),
            format: None,
            options: None,
            template: None,
            stream: None,
            keep_alive: None,
        }
    }
    
    pub fn messages(mut self, messages: Vec<Message>) -> Self {
        self.messages = messages;
        self
    }

    pub fn format(mut self, format: String) -> Self {
        self.format = Some(format);
        self
    }

    pub fn options(mut self, options: Value) -> Self {
        self.options = Some(options);
        self
    }

    pub fn template(mut self, template: String) -> Self {
        self.template = Some(template);
        self
    }

    pub fn stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
        self
    }

    pub fn keep_alive(mut self, keep_alive: String) -> Self {
        self.keep_alive = Some(keep_alive);
        self
    }

    pub fn build(self) -> ChatRequest {
        ChatRequest {
            model: self.model,
            messages: self.messages,
            format: self.format,
            options: self.options,
            template: self.template,
            stream: self.stream,
            keep_alive: self.keep_alive,
        }
    }
}

impl From<ChatRequestBuilder> for String {
    fn from(request: ChatRequestBuilder) -> Self {
        serde_json::to_string(&request.build()).unwrap()
    }
}