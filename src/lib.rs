pub mod llm_client;
pub mod models;
pub mod chat;
pub mod error;
pub mod ollama;
pub mod util;

pub use llm_client::LLMClient;
pub use models::GenerateResponse;
pub use models::GenerateRequest;
pub use models::GenerateRequestBuilder;
pub use chat::Chat;
pub use error::HiramuError;
pub use ollama::ollama_client::OllamaClient;
pub use util::fetch_and_base64_encode_image;
