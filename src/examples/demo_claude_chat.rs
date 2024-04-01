use crate::bedrock::models::claude::claude_client::ChatOptions;
use crate::bedrock::models::claude::claude_client::ClaudeClient;
use crate::bedrock::models::claude::claude_request_message::ConversationRequest;
use crate::bedrock::models::claude::claude_request_message::Message;

pub async fn demo_chat_claude() {
    let client = ClaudeClient::new("bedrock".to_string(), "us-west-2".to_string());

    let conversation_request = ConversationRequest {
        max_tokens: Some(100),
        anthropic_version: "bedrock-2023-05-31".to_string(),
        system: Some("Your are a useful assistant.".to_string()),
        messages: vec![
            Message::new_user_message("Hello, how are you?".to_owned()),
            Message::new_assistant_message("I'm doing well, thank you!".to_owned()),
            Message::new_user_message("That's great to hear!".to_owned()),
        ],
    };

    // print the request in JSON, beautifully displayed
    println!("{}", serde_json::to_string_pretty(&conversation_request).unwrap());

    let response = client
        .chat(
            &conversation_request,
            ChatOptions {
                model_id: "anthropic.claude-3-haiku-20240307-v1:0".to_string(),
                temperature: Some(0.5),
                top_p: Some(1.0),
                top_k: Some(50),
                max_tokens: 100,
                stop_sequences: Some(vec!["\n\nHuman:".to_string()]),
            },
        )
        .await;

    match response {
        Ok(response) => {
            println!("{:?}", response);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }

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
