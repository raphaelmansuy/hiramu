pub mod error;
pub mod ollama;
pub mod util;
pub mod bedrock;

pub mod examples;

pub use error::HiramuError;
pub use ollama::ollama_client::OllamaClient;
pub use ollama::models::{ GenerateRequest, GenerateRequestBuilder, GenerateResponse };
pub use util::fetch_and_base64_encode_image;

