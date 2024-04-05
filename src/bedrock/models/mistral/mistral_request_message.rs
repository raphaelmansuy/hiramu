use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MistralRequest {
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
}



#[derive(Debug, Deserialize,Serialize)]
pub struct MistralResponse {
    pub outputs: Vec<MistralOutput>,
}

#[derive(Debug, Deserialize,Serialize)]
pub struct MistralOutput {
    pub text: String,
    pub stop_reason: Option<String>,
}

pub struct MistralRequestBuilder {
    prompt: String,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
    top_p: Option<f32>,
    top_k: Option<u32>,
    stop: Option<Vec<String>>,
}

impl MistralRequestBuilder {
    pub fn new(prompt: String) -> Self {
        Self {
            prompt,
            max_tokens: None,
            temperature: None,
            top_p: None,
            top_k: None,
            stop: None,
        }
    }

    pub fn max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    pub fn temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    pub fn top_p(mut self, top_p: f32) -> Self {
        self.top_p = Some(top_p);
        self
    }

    pub fn top_k(mut self, top_k: u32) -> Self {
        self.top_k = Some(top_k);
        self
    }

    pub fn stop(mut self, stop: Vec<String>) -> Self {
        self.stop = Some(stop);
        self
    }

    pub fn build(self) -> MistralRequest {
        MistralRequest {
            prompt: self.prompt,
            max_tokens: self.max_tokens,
            temperature: self.temperature,
            top_p: self.top_p,
            top_k: self.top_k,
            stop: self.stop,
        }
    }
}

pub struct MistralOptionsBuilder {
    max_tokens: Option<u32>,
    temperature: Option<f32>,
    top_p: Option<f32>,
    top_k: Option<u32>,
    stop: Option<Vec<String>>,
}

impl Default for MistralOptionsBuilder {
    fn default() -> Self {
        Self {
            max_tokens: Some(400),
            temperature: Some(0.7),
            top_p: Some(0.7),
            top_k: Some(50),
            stop: None,
        }
    }
}

impl MistralOptionsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    pub fn temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    pub fn top_p(mut self, top_p: f32) -> Self {
        self.top_p = Some(top_p);
        self
    }

    pub fn top_k(mut self, top_k: u32) -> Self {
        self.top_k = Some(top_k);
        self
    }

    pub fn stop(mut self, stop: Vec<String>) -> Self {
        self.stop = Some(stop);
        self
    }

    pub fn build(self) -> MistralRequest {
        MistralRequest {
            prompt: String::new(),
            max_tokens: self.max_tokens,
            temperature: self.temperature,
            top_p: self.top_p,
            top_k: self.top_k,
            stop: self.stop,
        }
    }
}