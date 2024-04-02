use crate::ollama::model::{GenerateRequest, GenerateResponse, ChatRequest, ChatResponse};
use futures::stream::TryStreamExt;
use reqwest::{Client, RequestBuilder};
use serde::de::DeserializeOwned;
use futures::stream::TryStream;

use crate::error::HiramuError;

pub type FetchStreamError = HiramuError;

pub struct OllamaClient {
    client: Client,
    base_url: String,
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
        Ok(body.map_err(|e|   FetchStreamError::from(e)).and_then(|chunk| {
            async move {
                let chunk = serde_json::from_slice(&chunk).map_err(|e| FetchStreamError::from(e))?;
                Ok(chunk)
            }
        }))
    } else {
        Err(FetchStreamError::InvalidResponse(format!("HTTP error: {}", status)))
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

    // New chat method
    pub async fn chat(
        &self,
        request: ChatRequest,
    ) -> Result<impl TryStream<Ok = ChatResponse, Error = FetchStreamError>, FetchStreamError> {
        let url = format!("{}/api/chat", self.base_url);

        let request = self.client.post(&url).json(&request);

        let stream = fetch_stream::<ChatResponse>(request).await?;

        Ok(stream)
    }
}