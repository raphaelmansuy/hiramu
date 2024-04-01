use crate::bedrock::model_info::{ModelInfo, ModelName};
use crate::bedrock::models::claude::claude_client::ClaudeClient;
use crate::bedrock::models::claude::claude_request_message::ChatOptions;
use crate::bedrock::models::claude::claude_request_message::ConversationRequest;
use crate::bedrock::models::claude::claude_request_message::Message;

pub async fn demo_chat_claude() {
    let client = ClaudeClient::new("bedrock".to_string(), "us-west-2".to_string());

    let mut conversation_request = ConversationRequest::default();

    conversation_request
        .messages
        .push(Message::new_user_message(
            "What is the capital of France ?".to_owned(),
        ));

    println!(
        "{}",
        serde_json::to_string_pretty(&conversation_request).unwrap()
    );

    let response = client
        .chat(
            &conversation_request,
            ChatOptions {
                model_id: ModelInfo::from_model_name(ModelName::AnthropicClaudeHaiku1x),
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
            let json_display = serde_json::to_string_pretty(&response).unwrap();
            println!("{:?}", json_display);
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
