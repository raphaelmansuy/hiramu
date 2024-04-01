use crate::bedrock::models::claude::claude_client::CompletionOptions;
use crate::bedrock::models::claude::claude_client::ClaudeClient;

pub async fn demo_completion_claude() {
    let client = ClaudeClient::new("bedrock".to_string(), "us-west-2".to_string());

    let response = client
        .complete(
            "\n\nHuman:\nHi. In a short paragraph, explain what you can do.\n\nAssistant:",
            CompletionOptions {
                temperature: Some(0.5),
                top_p: Some(1.0),
                top_k: Some(50),
                max_tokens: 100,
                model_id: "anthropic.claude-3-haiku-20240307-v1:0".to_string(),
                stop_sequences: Some(vec!["\n\nHuman:".to_string()]),
            },
        )
        .await
        .unwrap();

    println!("{:?}", response);
}


// Write test
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_demo_completion_claude() {
        demo_completion_claude().await;
    }
}