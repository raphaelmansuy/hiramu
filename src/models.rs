use serde::{ Deserialize, Serialize };
use chrono::DateTime;
use chrono::Utc;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GenerateRequest {
    pub model: String,
    pub prompt: String,
    pub images: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GenerateResponse {
    pub model: String,
    pub created_at: DateTime<Utc>,
    pub response: String,
    pub done: bool,
}

pub struct GenerateRequestBuilder {
    model: String,
    prompt: String,
    images: Option<Vec<String>>,
}

impl GenerateRequestBuilder {
    pub fn new(model: String, prompt: String) -> Self {
        Self {
            model,
            prompt,
            images: None,
        }
    }

    pub fn images(mut self, images: Vec<String>) -> Self {
        self.images = Some(images);
        self
    }

    pub fn build(self) -> GenerateRequest {
        GenerateRequest {
            model: self.model,
            prompt: self.prompt,
            images: self.images,
        }
    }
}
