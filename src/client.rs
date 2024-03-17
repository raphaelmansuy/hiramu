use reqwest::Client;
use serde_json::Error as SerdeJsonError;
use thiserror::Error;
use futures_util::stream::{Stream, StreamExt};
use async_stream::stream;
use super::models::{GenerateRequest, GenerateResponse};
use super::error::HiramuError;
use bytes::Buf;

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

                // Process the buffer, splitting by newlines
                let mut offset = 0;
                while let Some(newline_idx) = buffer[offset..].iter().position(|&b| b == b'\n') {
                    let newline_idx = offset + newline_idx;
                    let line = &buffer[offset..newline_idx];
                    offset = newline_idx + 1; // Skip past the newline character

                    // Attempt to deserialize the JSON object
                    if let Ok(text) = String::from_utf8(line.to_vec()) {
                        match serde_json::from_str::<GenerateResponse>(&text) {
                            Ok(response) => {
                                let done = response.done; // Store the done value before moving `response`
                                println!("Received response: {:?}", response.response);
                                yield Ok(response); // `response` is moved here
                                if done {
                                    return;
                                }
                            },
                            Err(e) => {
                                println!("JSON parsing error: {:?}", e);
                                // Continue processing other lines, even if one line fails to parse
                            }
                        }
                    }
                }
                buffer.drain(..offset); // Remove processed lines from the buffer
            }
        }
    }
}