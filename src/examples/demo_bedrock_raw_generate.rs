use crate::bedrock::bedrock_client::BedrockClient;
use crate::bedrock::model_info::{ModelInfo, ModelName};

pub async fn demo_generate_raw() {
    let model_id = ModelInfo::from_model_name(ModelName::AnthropicClaudeHaiku1x);
    let profile_name = "bedrock";
    let region = "us-west-2";

    let prompt = "Hi. In a short paragraph, explain what you can do.";

    let payload = serde_json::json!({
        "anthropic_version": "bedrock-2023-05-31",
        "max_tokens": 1000,
        "messages": [{
            "role": "user",
            "content": [{
                "type": "text",
                "text": prompt
            }]
        }]
    });

    let client = BedrockClient::new();

    let result = client
        .generate_raw(
            model_id.to_string(),
            payload,
            Some(profile_name.to_string()),
            Some(region.to_string()),
        )
        .await
        .unwrap();

    println!("{:?}", result);
}

// Create a test

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_demo_generate_raw() {
        demo_generate_raw().await;
    }
}
