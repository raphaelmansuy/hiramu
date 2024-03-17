pub mod llm_client;
pub mod models;
pub mod chat;
pub mod error;
pub mod ollama;

pub use llm_client::LLMClient;
pub use models::GenerateResponse;
pub use models::GenerateRequest;
pub use chat::Chat;
pub use error::HiramuError;
pub use ollama::ollama_client::OllamaClient;


#[cfg(test)]
mod tests;
