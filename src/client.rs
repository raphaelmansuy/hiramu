use crate::error::HiramuError;
use crate::models::{GenerateRequest, GenerateResponse};
use reqwest::{Client};
use futures::StreamExt;

pub struct Hiramu {
    base_url: String,
    client: Client,
}

impl Hiramu {
    pub fn new(base_url: &str) -> Self {
        Hiramu {
            base_url: base_url.to_owned(),
            client: Client::new(),
        }
    }
    pub async fn generate(&self, model: &str, prompt: &str) -> Result<impl futures::Stream<Item = Result<GenerateResponse, HiramuError>>, HiramuError> {
        let request = GenerateRequest {
            model: model.to_string(),
            prompt: prompt.to_string(),
        };

        let url = format!("{}/api/generate", self.base_url);
        let response = self.client.post(url)
            .json(&request)
            .send()
            .await?;

        // Parse the JSON response here and then decide what to do with it
        let generate_response = response.json::<GenerateResponse>().await?;

        // Now you can use `generate_response` without trying to use `response` again
        // For example, you might want to return a stream of `GenerateResponse` objects
        // This is just a placeholder for your actual logic
        Ok(futures::stream::once(async move { Ok(generate_response) }))
    }
}