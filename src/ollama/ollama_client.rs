use crate::ollama::error::GenerateError;
use crate::ollama::models::{GenerateRequest, GenerateResponse};
use futures::StreamExt;
use reqwest::Client;
use std::error::Error;


pub struct OllamaClient {
    client: Client,
    base_url: String,
}

impl OllamaClient {
    /// Constructs a new `OllamaClient`.
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }

    pub async fn generate(
        &self,
        request: GenerateRequest,
    ) ->  Result<impl futures::Stream<Item = Result<GenerateResponse, GenerateError>>, Box<dyn Error>>{
        let url = format!("{}/api/generate", self.base_url);
        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .expect("Failed to send request");
    
            let stream = response
            .bytes_stream()
            .map(|chunk| {
                chunk
                    .map_err(|e| e.into())
                    .and_then(|chunk| {
                        let json_line = std::str::from_utf8(&chunk)?;
                        if json_line.trim().is_empty() {
                            Ok(None)
                        } else {
                            let generate_response = serde_json::from_str(json_line)?;
                            Ok(Some(generate_response))
                        }
                    })
            })
            .filter_map(|item| async move { item.transpose() });
    
        Ok(stream)
    }
}

