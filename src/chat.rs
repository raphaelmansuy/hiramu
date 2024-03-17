use super::models::GenerateResponse;
use super::HiramuClient;
use super::HiramuError;
use super::GenerateRequest;
use futures_util::stream::{ Stream }; // Import Stream trait
use std::pin::Pin; // For pinning the stream in the return type

pub struct Chat {
    client: HiramuClient,
    system_prompt: String,
}

impl Chat {
    pub fn new(client: HiramuClient, system_prompt: String) -> Self {
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
        let request = GenerateRequest {
            model: "mistral".to_string(),
            prompt: format!("{}: {}", self.system_prompt, message),
        };

        let response_stream = self.client.generate(request);

        // Box the stream and make it a dynamic Stream trait object with an explicit lifetime
        Box::pin(response_stream)
    }
}
