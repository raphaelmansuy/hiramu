use crate::ollama::model::{ChatRequest, ChatResponse, GenerateRequest, GenerateResponse};
use futures::stream::TryStream;
use futures::stream::TryStreamExt;
use reqwest::{Client, RequestBuilder};
use serde::de::DeserializeOwned;

use super::error::OllamaError;
use crate::ollama::model::{EmbeddingsRequest, EmbeddingsResponse};

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

    pub async fn embeddings(
        &self,
        request: EmbeddingsRequest,
    ) -> Result<EmbeddingsResponse, OllamaError> {
        let url = format!("{}/api/embeddings", self.base_url);

        let response = self.client.post(&url).json(&request).send().await?;

        let status = response.status();
        let body = response.text().await?;

        if status.is_success() {
            let embedding_response: EmbeddingsResponse =
                serde_json::from_str(&body).map_err(OllamaError::from)?;
            Ok(embedding_response)
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
}

#[cfg(test)]
mod tests {
    use crate::ollama::EmbeddingsRequestBuilder;

    use super::*;

    #[tokio::test]
    async fn test_embeddings() {
        let client = OllamaClient::new("http://localhost:11434".to_string());

        let request = crate::ollama::EmbeddingsRequestBuilder::new(
            "nomic-embed-text".to_string(),
            "Here is an article about llamas...".to_string(),
        )
        .options(serde_json::json!({ "temperature": 0.8 }))
        .keep_alive("10m".to_string())
        .build();

        let response = client.embeddings(request).await;

        let response = match response {
            Ok(response) => response,
            Err(err) => panic!("Error: {:?}", err),
        };

        print!("Embeddings: {:?}", response);

        assert_eq!(response.embedding.len(), 768);
    }

    #[tokio::test]
    async fn test_embeddings_builder() {
        let json_request: String = EmbeddingsRequestBuilder::new(
            "all-minilm".to_string(),
            "Here is an article about llamas...".to_string(),
        )
        .options(serde_json::json!({ "temperature": 0.8 }))
        .keep_alive("10m".to_string())
        .into();

        let expected_json = r#"{"model":"all-minilm","prompt":"Here is an article about llamas...","options":{"temperature":0.8},"keep_alive":"10m"}"#;
        assert_eq!(json_request, expected_json);
    }

    #[tokio::test]
    async fn test_embeddings_error() {
        let client = OllamaClient::new("http://localhost:11434".to_string());

        let request = EmbeddingsRequestBuilder::new(
            "invalid-model".to_string(),
            "Here is an article about llamas...".to_string(),
        )
        .build();

        let result = client.embeddings(request).await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), OllamaError::NotFound(_)));
    }
}
