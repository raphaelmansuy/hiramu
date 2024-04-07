use crate::bedrock::bedrock_client::{BedrockClient, BedrockClientOptions};
use crate::bedrock::models::mistral::error::MistralError;
use crate::bedrock::models::mistral::mistral_request_message::{MistralRequest, MistralResponse};
use futures::stream::Stream;
use futures::TryStreamExt;

pub type MistralOptions = BedrockClientOptions;

pub struct MistralClient {
    client: BedrockClient,
}

impl MistralClient {
    /// Constructs a new `MistralClient`.
    pub async fn new(options: MistralOptions) -> Self {
        Self {
            client: BedrockClient::new(options).await,
        }
    }

    /// Generates a response from the Mistral model.
    pub async fn generate(
        &self,
        model_id: String,
        request: &MistralRequest,
    ) -> Result<MistralResponse, MistralError> {
        let payload = serde_json::to_value(request).map_err(MistralError::Json)?;

        let response = self.client.generate_raw(model_id, payload).await?;

        let mistral_response = serde_json::from_value(response).map_err(MistralError::Json)?;
        Ok(mistral_response)
    }

    /// Generates a stream of responses from the Mistral model.
    pub async fn generate_with_stream(
        &self,
        model_id: String,
        request: &MistralRequest,
    ) -> Result<impl Stream<Item = Result<MistralResponse, MistralError>>, MistralError> {
        let payload = serde_json::to_value(request).map_err(MistralError::Json)?;

        let response = self.client.generate_raw_stream(model_id, payload).await?;


        Ok(response
            .map_ok(|value| serde_json::from_value(value).map_err(MistralError::Json))
            .map_err(|err| MistralError::Bedrock(err))
            .and_then(futures::future::ready))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::bedrock::{models::mistral::mistral_request_message::MistralRequestBuilder, ModelInfo};
    use futures::stream::StreamExt;

    #[tokio::test]
    async fn test_generate() {
        let options = MistralOptions::new().profile_name("bedrock").region("us-west-2");
        let client = MistralClient::new(options).await;

        let request = MistralRequestBuilder::new("<s>[INST] What is the capital of France ?[/INST]".to_string())
            .max_tokens(200)
            .temperature(0.8)
            .build();

        let model_name = ModelInfo::from_model_name(crate::bedrock::ModelName::MistralMixtral8X7BInstruct0x);

        let response = client.generate(model_name, &request).await;

        let response = match response {
            Ok(response) => response,
            Err(err) => panic!("Error: {:?}", err),
        };

        println!("Response: {:?}", response.outputs[0].text.to_string());

        assert!(!response.outputs.is_empty());
    }

    #[tokio::test]
    async fn test_generate_with_stream() {
        let options = MistralOptions::new().profile_name("bedrock").region("us-west-2");
        let client = MistralClient::new(options).await;

        let request = MistralRequestBuilder::new("<s>[INST] What is the capital of France ?[/INST]".to_string())
            .max_tokens(200)
            .temperature(0.8)
            .build();

       let model_name = ModelInfo::from_model_name(crate::bedrock::ModelName::MistralMixtral8X7BInstruct0x);

        // display the request as a pretty-printed JSON string
        let display_request = serde_json::to_string_pretty(&request).unwrap();
        println!("Request: {}", display_request);



        let mut stream = client
            .generate_with_stream(model_name.to_owned(), &request)
            .await
            .unwrap();

        let mut response_text = String::new();
        while let Some(result) = stream.next().await {
            match result {
                Ok(response) => {
                    println!("Response: {:?}", response.outputs[0].text.to_string());
                    response_text.push_str(&response.outputs[0].text);
                }
                Err(err) => {
                    panic!("Error: {:?}", err);
                }
            }
        }

        assert!(!response_text.is_empty());

    }


}