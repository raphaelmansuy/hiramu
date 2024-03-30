use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use serde_json::Value;
use pin_project::pin_project;

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

impl TryFrom<&str> for GenerateResponse {
    type Error = serde_json::Error;
    
    fn try_from(json: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(json)
    }
}

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
    
    pub fn prompt(mut self, prompt: String) -> Self {
        self.prompt = Some(prompt);
        self
    }
    
    pub fn images(mut self, images: Vec<String>) -> Self {
        self.images = Some(images);
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

    pub fn system(mut self, system: String) -> Self {
        self.system = Some(system);
        self
    }

    pub fn template(mut self, template: String) -> Self {
        self.template = Some(template);
        self
    }

    pub fn context(mut self, context: Vec<u32>) -> Self {
        self.context = Some(context);
        self
    }

    pub fn stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
        self
    }

    pub fn raw(mut self, raw: bool) -> Self {
        self.raw = Some(raw);
        self
    }

    pub fn keep_alive(mut self, keep_alive: String) -> Self {
        self.keep_alive = Some(keep_alive);
        self
    }

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

impl From<GenerateRequestBuilder> for String {
    fn from(request: GenerateRequestBuilder) -> Self {
        serde_json::to_string(&request.build()).unwrap()
    }
}

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

#[derive(Debug, Serialize,Deserialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub images: Vec<String>,
}

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
    type Error = serde_json::Error;
    
    fn try_from(json: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(json)
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