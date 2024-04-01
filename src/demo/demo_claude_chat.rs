use crate::bedrock::models::claude::claude_client::ClaudeClient;
use crate::bedrock::models::claude::claude_request_message::Message;
use crate::bedrock::models::claude::claude_request_message::Content;
use crate::bedrock::models::claude::claude_client::ChatOptions;

pub async fn demo_chat_claude() {
    let client = ClaudeClient::new("bedrock".to_string(), "us-west-2".to_string());

    let chat_message = Message {
        role: "user".to_string(),
        content: Content::Text("Hi. In a short paragraph, explain what you can do.".to_string()),
    };

    let response = client
        .chat(
            &[chat_message],
            ChatOptions {
                model_id: "anthropic.claude-3-haiku-20240307-v1:0".to_string(),
                temperature: Some(0.5),
                top_p: Some(1.0),
                top_k: Some(50),
                max_tokens: 100,
                stop_sequences: Some(vec!["\n\nHuman:".to_string()]),
            },
        )
        .await
        .unwrap();

    println!("{:?}", response);
}

// Test 
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_demo_chat_claude() {
        demo_chat_claude().await;
    }
}