
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use serde_json::Value;
use pin_project::pin_project;

use super::error::OllamaError;
use crate::ollama::options::OptionsBuilder;

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

fn merge_options(options: Option<Value>, builder_options: Option<Value>) -> Value {
    match (options, builder_options) {
        (Some(options), Some(options_builder)) => {
            let mut merged_options = options.as_object().unwrap().clone();
            let options_builder = options_builder.as_object().unwrap();
            for (key, value) in options_builder {
                merged_options.insert(key.clone(), value.clone());
            }
            serde_json::Value::Object(merged_options)
        },
        (Some(options), None) => options,
        (None, Some(options_builder)) => options_builder,
        (None, None) => serde_json::Value::Object(serde_json::Map::new()),
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
    options_builder: Option<OptionsBuilder>
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
            options_builder: None,
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



    pub fn options_from_builder(mut self, options_builder: OptionsBuilder) -> Self {
        self.options = Some(serde_json::to_value(options_builder.build()).unwrap());
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
        let options = self.options_builder.map(|builder| builder.build());
        let options = options.map(|options| serde_json::to_value(options).unwrap());
        let merged_options = merge_options(self.options, options);

        GenerateRequest {
            model: self.model,
            prompt: self.prompt,
            images: self.images.unwrap_or_default(),
            format: self.format,
            options: Some(merged_options),
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
    options_builder: Option<OptionsBuilder>,
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
            options_builder: None,
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



    pub fn options_from_builder(mut self, options_builder: OptionsBuilder) -> Self {
        self.options = Some(serde_json::to_value(options_builder.build()).unwrap());
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

        let options = self.options_builder.map(|builder| builder.build());
        let options = options.map(|options| serde_json::to_value(options).unwrap());
        let merged_options = merge_options(self.options, options);

        ChatRequest {
            model: self.model,
            messages: self.messages,
            format: self.format,
            options: Some(merged_options),
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

#[derive(Debug, Serialize)]
pub struct EmbeddingsRequest {
    pub model: String,
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_alive: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EmbeddingsResponse {
    pub embedding: Vec<f32>,
}


pub struct EmbeddingsRequestBuilder {
    model: String,
    prompt: String,
    options: Option<Value>,
    keep_alive: Option<String>,
}

impl EmbeddingsRequestBuilder {
    pub fn new(model: String, prompt: String) -> Self {
        Self {
            model,
            prompt,
            options: None,
            keep_alive: None,
        }
    }

    pub fn options(mut self, options: Value) -> Self {
        self.options = Some(options);
        self
    }

    pub fn keep_alive(mut self, keep_alive: String) -> Self {
        self.keep_alive = Some(keep_alive);
        self
    }

    pub fn build(self) -> EmbeddingsRequest {
        EmbeddingsRequest {
            model: self.model,
            prompt: self.prompt,
            options: self.options,
            keep_alive: self.keep_alive,
        }
    }
}

impl From<EmbeddingsRequestBuilder> for String {
    fn from(request: EmbeddingsRequestBuilder) -> Self {
        serde_json::to_string(&request.build()).unwrap()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_merge_options_both_some() {
        let options = json!({"key1": "value1"});
        let builder_options = json!({"key2": "value2"});
        let result = merge_options(Some(options), Some(builder_options));
        assert_eq!(result, json!({"key1": "value1", "key2": "value2"}));
    }

    #[test]
    fn test_merge_options_only_options_some() {
        let options = json!({"key1": "value1"});
        let result = merge_options(Some(options), None);
        assert_eq!(result, json!({"key1": "value1"}));
    }

    #[test]
    fn test_merge_options_only_builder_options_some() {
        let builder_options = json!({"key2": "value2"});
        let result = merge_options(None, Some(builder_options));
        assert_eq!(result, json!({"key2": "value2"}));
    }

    #[test]
    fn test_merge_options_both_none() {
        let result = merge_options(None, None);
        assert_eq!(result, json!({}));
    }
}