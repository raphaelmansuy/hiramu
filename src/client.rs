use reqwest::Client;
use serde::Deserialize;
use serde_json::Error as SerdeJsonError;
use thiserror::Error;
use futures_util::stream::{self, Stream, StreamExt};
use async_stream::stream;
use super::models::{GenerateRequest, GenerateResponse};
use super::error::HiramuError;

pub struct HiramuClient {
    client: Client,
    base_url: String,
}

impl HiramuClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }
    pub fn generate(&self, request: GenerateRequest) -> impl Stream<Item = Result<GenerateResponse, HiramuError>> + '_ {
        let url = format!("{}/api/generate", self.base_url);
        let client = self.client.clone();

        stream! {
            let response = match client.post(&url)
                .json(&request)
                .send()
                .await {
                    Ok(res) => res,
                    Err(e) => {
                        yield Err(HiramuError::Http(e));
                        return;
                    }
                };

            let body = match response.error_for_status() {
                Ok(body) => body,
                Err(e) => {
                    yield Err(HiramuError::Http(e));
                    return;
                }
            };

            println!("Received response: {:?}", body);

            let mut stream = body.bytes_stream();
            let mut buffer = Vec::new();

            while let Some(chunk) = stream.next().await {
                let chunk = match chunk {
                    Ok(c) => c,
                    Err(e) => {
                        yield Err(HiramuError::Http(e));
                        return;
                    }
                };
                buffer.extend_from_slice(&chunk);


                // Attempt to deserialize when a valid JSON object is formed
                if let Ok(text) = String::from_utf8(buffer.clone()) {
                    if let Ok(responses) = serde_json::from_str::<Vec<GenerateResponse>>(&text) {
                        for response in responses {
                            println!("Received response: {:?}", response.response);
                            // Clone the response to avoid moving it
                            yield Ok(response.clone());
                            if response.done {
                                return;
                            }
                        }
                        buffer.clear();
                    }
                }
            }
        }
    }
}