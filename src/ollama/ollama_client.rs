use crate::ollama::models::{GenerateRequest, GenerateResponse};
use futures::stream::{TryStreamExt};
use reqwest::{Client, RequestBuilder};
use serde::{de::DeserializeOwned};
use thiserror::Error;
use futures::stream::TryStream;

pub struct OllamaClient {
    client: Client,
    base_url: String,
}

#[derive(Error, Debug)]
pub enum FetchStreamError {
    /// Request failed with the specified status code.
    #[error("Request failed with status: {0}")]
    RequestFailed(reqwest::StatusCode),
    /// Failed to deserialize the received data.
    #[error("Failed to deserialize data: {0}")]
    DeserializationFailed(serde_json::Error),
    /// An error occurred during the request.
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
}

/// Fetches a stream of data from the specified URL and deserializes it into chunks of type `T`.
///
/// # Arguments
///
/// * `url` - The URL to fetch the data from.
/// * `config` - The configuration options for the request.
///
/// # Returns
///
/// A `Result` containing a `TryStream` of `Chunk<T>` if the request is successful,
/// or a `FetchStreamError` if an error occurs.
async fn fetch_stream<T>(request: RequestBuilder) -> Result<impl TryStream<Ok = T, Error = FetchStreamError>, FetchStreamError>
where
    T: DeserializeOwned,
{ 
    let response = request.send().await?;

    let status = response.status();
    let body = response.bytes_stream();

    if status.is_success() {
        Ok(body.map_err(FetchStreamError::RequestError).and_then(|chunk| {
            async move {
                let chunk = serde_json::from_slice(&chunk).map_err(FetchStreamError::DeserializationFailed)?;
                Ok(chunk)
            }
        }))
    } else {
        Err(FetchStreamError::RequestFailed(status))
    }
}

impl OllamaClient {
    /// Constructs a new `OllamaClient`.
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }

    // the function signature will return either a stream of `GenerateResponse` encapsulated in a Result or a  `GenerateError`
    pub async fn generate(
        &self,
        request: GenerateRequest,
    ) -> Result<impl TryStream<Ok = GenerateResponse, Error = FetchStreamError>, FetchStreamError> {
        let url = format!("{}/api/generate", self.base_url);

        
        let request = self.client
            .post(&url)
            .json(&request);

        let stream = fetch_stream::<GenerateResponse>(request).await?;

        return Ok(stream);
    }
}