// disable warning for unused import for StreamExt
// #[allow(unused_imports)]
use futures_util::stream::{ Stream };

use super::models::{ GenerateRequest, GenerateResponse };
use super::error::HiramuError;
use std::pin::Pin;

pub trait LLMClient {
    fn generate(
        &self,
        request: GenerateRequest
    ) -> Pin<Box<dyn Stream<Item = Result<GenerateResponse, HiramuError>> + Send>>;

    fn default_llm_model(&self) -> String;
}
