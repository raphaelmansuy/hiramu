use std::io::Write;

use futures::TryStreamExt;

use crate::bedrock::model_info::{ModelInfo, ModelName};
use crate::bedrock::models::claude::claude_client::{ClaudeClient, ClaudeOptions};
use crate::bedrock::models::claude::claude_request_message::{
    ChatOptions, ContentBlockDelta, ConversationRequest, Message, StreamResultData,
};

pub async fn chat_with_claude() {
    let claude_options = ClaudeOptions::new()
        .profile_name("bedrock")
        .region("us-west-2");

    let client = ClaudeClient::new(claude_options).await;

    let mut conversation_request = ConversationRequest::default();
    conversation_request
        .messages
        .push(Message::new_user_message("Hello, Claude!".to_owned()));

    let chat_options = ChatOptions::default()
        .with_temperature(0.7)
        .with_max_tokens(100)
        .with_model_id(ModelInfo::from_model_name(
            ModelName::AnthropicClaudeHaiku1x,
        ));

    let response_stream = client
        .chat_with_stream(&conversation_request, &chat_options)
        .await
        .unwrap();

    response_stream
        .try_for_each(|chunk| async move {
            match chunk {
                StreamResultData::ContentBlockStart(..) => {
                    println!("\n------------------------------");
                }
                StreamResultData::ContentBlockStop(..) => {
                    println!("\n------------------------------");
                }

                StreamResultData::ContentBlockDelta(ContentBlockDelta { delta, .. }) => {
                    print!("{}", delta.text);
                    std::io::stdout().flush().unwrap();
                }
                _ => {}
            }
            Ok(())
        })
        .await
        .unwrap();
}

// Main
#[tokio::main]
pub async fn main() {
    chat_with_claude().await;
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_chat_with_claude() {
        chat_with_claude().await;
    }
}
