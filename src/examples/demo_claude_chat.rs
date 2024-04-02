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

    let chat_options =
        ChatOptions::default()
            .with_temperature(0.5)
            .with_model_id(ModelInfo::from_model_name(
                ModelName::AnthropicClaudeHaiku1x,
            ));

    let response = client.chat(&conversation_request, &chat_options).await;

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
