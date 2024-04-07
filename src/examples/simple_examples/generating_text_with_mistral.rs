use crate::bedrock::model_info::{ModelInfo, ModelName};
use crate::bedrock::models::mistral::mistral_client::{MistralClient, MistralOptions};
use crate::bedrock::models::mistral::mistral_request_message::MistralRequestBuilder;

async fn generating_text_with_mistral() {
    let mistral_options = MistralOptions::new()
        .profile_name("bedrock")
        .region("us-west-2");

    let client = MistralClient::new(mistral_options).await;

    let request =
        MistralRequestBuilder::new("<s>[INST] What is the capital of France?[/INST]".to_string())
            .max_tokens(200)
            .temperature(0.8)
            .build();

    let model_id = ModelInfo::from_model_name(ModelName::MistralMixtral8X7BInstruct0x);
    let response = client.generate(model_id, &request).await.unwrap();

    println!("Response: {:?}", response.outputs[0].text);
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
