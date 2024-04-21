pub mod ollama_client;
pub mod model;
pub mod error;
pub mod options;

pub use error::OllamaError;
pub use ollama_client::OllamaClient;
pub use model::{ GenerateRequest, GenerateRequestBuilder, GenerateResponse };
pub use model::{ ChatRequest, ChatRequestBuilder, ChatResponse, Message };
pub use model::{ EmbeddingsRequest,EmbeddingsResponse, EmbeddingsRequestBuilder};
pub use options::OptionsBuilder;