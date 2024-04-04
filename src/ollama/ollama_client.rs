use crate::ollama::model::{ChatRequest, ChatResponse, GenerateRequest, GenerateResponse};
use futures::stream::TryStream;
use futures::stream::TryStreamExt;
use reqwest::{Client, RequestBuilder};
use serde::de::DeserializeOwned;

use super::error::OllamaError;


pub struct OllamaClient {
    client: Client,
    base_url: String,
}

async fn fetch_stream<T>(
    request: RequestBuilder,
) -> Result<impl TryStream<Ok = T, Error = OllamaError>, OllamaError>
where
    T: DeserializeOwned,
{
    let response = request.send().await?;

    let status = response.status();
    let body = response.bytes_stream();

    if status.is_success() {
        Ok(body
            .map_err(OllamaError::from)
            .and_then(|chunk| async move {
                let chunk = serde_json::from_slice(&chunk).map_err(OllamaError::from)?;
                Ok(chunk)
            }))
    } else {
        let message = format!("API request failed with status code: {}", status);
        match status.as_u16() {
            400 => Err(OllamaError::BadRequest(message)),
            401 => Err(OllamaError::Unauthorized(message)),
            403 => Err(OllamaError::Forbidden(message)),
            404 => Err(OllamaError::NotFound(message)),
            429 => Err(OllamaError::TooManyRequests(message)),
            500 => Err(OllamaError::InternalServerError(message)),
            _ => Err(OllamaError::UnknownApiError(message)),
        }
    }
}

impl OllamaClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }

    pub async fn generate(
        &self,
        request: GenerateRequest,
    ) -> Result<impl TryStream<Ok = GenerateResponse, Error = OllamaError>, OllamaError> {
        let url = format!("{}/api/generate", self.base_url);

        let request = self.client.post(&url).json(&request);

        let stream = fetch_stream::<GenerateResponse>(request).await?;

        Ok(stream)
    }

    pub async fn chat(
        &self,
        request: ChatRequest,
    ) -> Result<impl TryStream<Ok = ChatResponse, Error = OllamaError>, OllamaError> {
        let url = format!("{}/api/chat", self.base_url);

        let request = self.client.post(&url).json(&request);

        let stream = fetch_stream::<ChatResponse>(request).await?;

        Ok(stream)
    }
}
