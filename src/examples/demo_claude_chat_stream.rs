use futures::TryStreamExt;

use crate::bedrock::model_info::{ModelInfo, ModelName};
use crate::bedrock::models::claude::claude_client::{ClaudeClient, ClaudeOptions};
use crate::bedrock::models::claude::claude_request_message::ConversationRequest;
use crate::bedrock::models::claude::claude_request_message::Message;
use crate::bedrock::models::claude::claude_request_message::{ChatOptions, StreamResultData};

pub async fn demo_chat_claude_with_stream() {
    let claude_options = ClaudeOptions::new()
        .profile_name("bedrock")
        .region("us-west-2");

    let client = ClaudeClient::new(claude_options).await;

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
        .with_model_id(ModelInfo::from_model_name(
            ModelName::AnthropicClaudeHaiku1x,
        ))
        .with_temperature(0.5);

    let response_stream = client
        .chat_with_stream(&conversation_request, &chat_options)
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
            display_streamresult_data(chunk);
            Ok(())
        })
        .await
        .unwrap();
}

fn display_streamresult_data(data: StreamResultData) {
    match data {
        StreamResultData::ContentBlockStart(content_block_start) => {
            println!("ContentBlockStart: {:?}", content_block_start);
        }
        StreamResultData::ContentBlockStop(content_block_end) => {
            println!("ContentBlockEnd: {:?}", content_block_end);
        }
        StreamResultData::MessageStart(message_start) => {
            println!("MessageStart: {:?}", message_start);
        }
        StreamResultData::MessageStop(message_end) => {
            println!("MessageStop: {:?}", message_end);
        }
        StreamResultData::MessageDelta(message_delta) => {
            println!("MessageDelta: {:?}", message_delta);
        }
        StreamResultData::ContentBlockStart(content_block_start) => {
            println!("ContentBlockStart: {:?}", content_block_start);
        }
        StreamResultData::ContentBlockStop(content_block_end) => {
            println!("ContentBlockEnd: {:?}", content_block_end);
        }
        StreamResultData::ContentBlockDelta(content_block_delta) => {
            println!("ContentBlockDelta: {:?}", content_block_delta);
        }
    }
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
