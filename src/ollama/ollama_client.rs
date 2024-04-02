use crate::ollama::model::{GenerateRequest, GenerateResponse, ChatRequest, ChatResponse};
use futures::stream::TryStreamExt;
use reqwest::{Client, RequestBuilder};
use serde::de::DeserializeOwned;
use futures::stream::TryStream;

use crate::error::HiramuError;

pub type FetchStreamError = HiramuError;
/// Represents a client for interacting with the Ollama API.
///
/// This struct encapsulates the HTTP client and the base URL for the Ollama API.
/// It provides methods for making requests to the API, such as getting, creating, updating, and deleting resources.
///
/// # Fields
///
/// * `client` - The HTTP client used for making requests to the API.
/// * `base_url` - The base URL for the Ollama API. All requests will be made to this URL.
///
/// # Examples
///
/// ```
/// let client = Client::new();
/// let base_url = "https://localhost:11434";
/// let ollama_client = OllamaClient { client, base_url };
/// ```
pub struct OllamaClient {
    client: Client,
    base_url: String,
}

/// Fetches a data stream from the provided `RequestBuilder` and deserializes the
/// response into the specified type `T`.
///
/// This function sends the HTTP request and then handles the response. If the
/// response has a successful status code, it returns a `TryStream` that
/// deserializes each chunk of the response body into the target type `T`. If
/// the response has an error status code, it returns a `FetchStreamError`
/// containing the HTTP status code.
///
/// # Parameters
/// - `request`: A `RequestBuilder` representing the HTTP request to be sent.
///
/// # Returns
/// - On success, a `TryStream` that yields deserialized values of type `T`.
/// - On failure, a `FetchStreamError` containing the HTTP status code.
///
/// # Errors
/// This function can fail for various reasons, including:
/// - Network errors during the request
/// - Deserialization errors for the response body
/// - Invalid HTTP status codes in the response
///
/// # Example
/// ```
/// let request = reqwest::Client::new().get("https://example.com/api/data");
/// let stream: impl TryStream<Ok = MyData, Error = FetchStreamError> =
///     fetch_stream(request).await?;
/// async_stream::try_collect::<Vec<_>, _>(stream).await?;
/// ```
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
    /// Constructs a new `OllamaClient` with the provided base URL.
    ///
    /// The `OllamaClient` is a wrapper around a `reqwest::Client` that provides
    /// a high-level interface for interacting with the Ollama API. It handles
    /// the construction of the HTTP requests and the deserialization of the
    /// responses.
    ///
    /// # Parameters
    /// - `base_url`: The base URL of the Ollama API endpoint.
    ///
    /// # Returns
    /// A new `OllamaClient` instance.
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }

    /// Generates a new text response using the Ollama API.
    ///
    /// This method sends a `GenerateRequest` to the Ollama API and returns a
    /// `TryStream` that yields `GenerateResponse` values. If an error occurs
    /// during the request or response processing, a `FetchStreamError` is
    /// returned.
    ///
    /// # Parameters
    /// - `request`: The `GenerateRequest` containing the input text and
    ///   generation parameters.
    ///
    /// # Returns
    /// - On success, a `TryStream` that yields `GenerateResponse` values.
    /// - On failure, a `FetchStreamError` containing information about the
    ///   error.
    /// # Example
    /// ```
    /// let client = OllamaClient::new("https://localhost:11434".to_string());
    /// let request = GenerateRequest {
    ///    input: "Hello, world!".to_string(),
    ///   max_tokens: 50,
    /// };
    /// let stream = client.generate(request).await?;
    /// stream.try_for_each(|response| async {
    ///    println!("{}", response.output);
    ///   Ok(())
    /// }).await?;
    /// ```
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

    /// Initiates a chat session with the Ollama API.
    ///
    /// This method sends a `ChatRequest` to the Ollama API and returns a
    /// `TryStream` that yields `ChatResponse` values. If an error occurs
    /// during the request or response processing, a `FetchStreamError` is
    /// returned.
    ///
    /// # Parameters
    /// - `request`: The `ChatRequest` containing the chat input and parameters.
    ///
    /// # Returns
    /// - On success, a `TryStream` that yields `ChatResponse` values.
    /// - On failure, a `FetchStreamError` containing information about the
    ///   error.
    /// # Example
    /// ```
    /// let client = OllamaClient::new("https://localhost:11434".to_string());
    /// let request = ChatRequest {
    ///    input: "Hello, Ollama!".to_string(),
    /// };  
    /// let stream = client.chat(request).await?;
    /// stream.try_for_each(|response| async {
    ///    println!("{}", response.output);
    ///   Ok(())
    /// }).await?;
    /// ```
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