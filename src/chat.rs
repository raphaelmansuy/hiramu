use super::models::GenerateResponse;
use super::LLMClient;
use super::HiramuError;
use super::{ GenerateRequestBuilder };
use futures_util::stream::{ Stream }; // Import Stream trait
use std::pin::Pin; // For pinning the stream in the return type

pub struct Chat<'a> {
    client: &'a dyn LLMClient,
    system_prompt: String,
}

impl<'a> Chat<'a> {
    pub fn new(client: &'a dyn LLMClient, system_prompt: String) -> Self {
        Self {
            client,
            system_prompt,
        }
    }

    // Add the explicit `'_` lifetime bound to the return type
    pub fn add_message(
        &mut self,
        message: String
    ) -> Pin<Box<dyn Stream<Item = Result<GenerateResponse, HiramuError>> + Send + '_>> {
        
        let request = GenerateRequestBuilder::new(
            self.client.get_default_llm_model(),
            format!("{}: {}", self.system_prompt, message)
        ).build();

        let response_stream = self.client.generate(request);

        // Box the stream and make it a dynamic Stream trait object with an explicit lifetime
        // and ensure it is Send
        Box::pin(response_stream)
    }
}
