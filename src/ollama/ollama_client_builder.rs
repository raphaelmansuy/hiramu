pub use crate::ollama::ollama_client::OllamaClient;

const DEFAULT_LLM_MODEL: &str = "mistral";
const DEFAULT_BASE_URL: &str = "http://localhost:11434";

pub struct OllamaClientBuilder {
    base_url: Option<String>,
    default_llm_model: Option<String>,
}

impl OllamaClientBuilder {
    pub fn new() -> Self {
        Self {
            base_url: None,
            default_llm_model: None,
        }
    }

    pub fn url(mut self, base_url: &str) -> Self {
        self.base_url = Some(base_url.to_string());
        self
    }

    pub fn default_llm_model(mut self, model: &str) -> Self {
        self.default_llm_model = Some(model.to_string());
        self
    }

    pub fn build(self) -> OllamaClient {
        OllamaClient::new(
            reqwest::Client::new(),
            self.base_url.unwrap_or_else(|| DEFAULT_BASE_URL.to_string()),
            self.default_llm_model.unwrap_or_else(|| DEFAULT_LLM_MODEL.to_string())
        )
    }
}
