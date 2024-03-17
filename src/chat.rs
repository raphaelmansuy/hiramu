use super::models::GenerateResponse;
use super::HiramuClient;
use super::HiramuError;
use super::GenerateRequest;
use futures_util::stream::StreamExt; // For using .next()
use pin_utils::pin_mut; // To pin the stream

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

    pub async fn add_message(&mut self, message: String) -> Result<GenerateResponse, HiramuError> {
        let request = GenerateRequest {
            model: "mistral".to_string(),
            prompt: format!("{}: {}", self.system_prompt, message),
        };

        let response_stream = self.client.generate(request);
        pin_mut!(response_stream); // Pin the stream to the stack

        // Initialize an empty response object
        let mut final_response = GenerateResponse {
            model: "mistral".to_string(),
            created_at: chrono::Utc::now().into(),
            response: String::new(),
            done: false,
        };

        // Process the stream
        while let Some(response_result) = response_stream.next().await {
            match response_result {
                Ok(response) => {
                    // Print the response for debugging or logging
                    println!("Received response: {:?}", response.response);

                    // Check if the response is marked as done
                    if response.done {
                        final_response = response;
                        break; // Exit the loop if done
                    } else {
                        // Update the final response with the latest one
                        final_response = response;
                    }
                },
                Err(e) => {
                    // Handle any errors that occurred while processing the stream
                    return Err(e);
                }
            }
        }

        // Return the final response object
        Ok(final_response)
    }
}