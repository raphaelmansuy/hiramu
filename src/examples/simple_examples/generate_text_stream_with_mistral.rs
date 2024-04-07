use futures::stream::StreamExt;
use crate::bedrock::models::mistral::mistral_client::{MistralClient, MistralOptions};
use crate::bedrock::models::mistral::mistral_request_message::MistralRequestBuilder;
use crate::bedrock::model_info::{ModelInfo, ModelName};

pub async fn generating_text_with_mistral() {
    let mistral_options = MistralOptions::new()
        .profile_name("bedrock")
        .region("us-west-2");

    let client = MistralClient::new(mistral_options).await;

    let request = MistralRequestBuilder::new("<s>[INST] What is the capital of France?[/INST]".to_string())
        .max_tokens(200)
        .temperature(0.8)
        .build();

    let model_id = ModelInfo::from_model_name(ModelName::MistralMixtral8X7BInstruct0x);
    let mut stream = client.generate_with_stream(model_id, &request).await.unwrap();

    while let Some(result) = stream.next().await {
        match result {
            Ok(response) => {
                println!("Response: {:?}", response.outputs[0].text);
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
            }
        }
    }
}

#[tokio::main]
pub async fn main() {
    generating_text_with_mistral().await;
}


// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generating_text_with_mistral() {
        generating_text_with_mistral().await;
    }
}