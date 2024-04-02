use futures::TryStreamExt;

use crate::bedrock::model_info::{ModelInfo, ModelName};
use crate::bedrock::models::claude::claude_client::ClaudeClient;
use crate::bedrock::models::claude::claude_request_message::ChatOptions;
use crate::bedrock::models::claude::claude_request_message::ConversationRequest;
use crate::bedrock::models::claude::claude_request_message::Message;

pub async fn demo_chat_claude_with_stream() {
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

    let chat_options = ChatOptions::default()
        .with_model_id(ModelInfo::from_model_name(ModelName::AnthropicClaudeHaiku1x))
        .with_temperature(0.5);

    let response_stream = client
        .chat_with_stream(
            &conversation_request,
            &chat_options
        )
        .await;

    let response_stream = match response_stream {
        Ok(response_stream) => response_stream,
        Err(e) => {
            println!("Error: {:?}", e);
            return;
        }
    };

    // consumme the stream and print the response
    response_stream
        .try_for_each(|chunk| async move {
            let json_display = serde_json::to_string_pretty(&chunk).unwrap();
            println!("{:?}", json_display);
            Ok(())
        })
        .await
        .unwrap();

}

// Test
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_demo_chat_claude_with_stream() {
        demo_chat_claude_with_stream().await;
    }
}
