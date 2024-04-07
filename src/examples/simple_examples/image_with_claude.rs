use std::io::Write;

use futures::TryStreamExt;

use crate::bedrock::models::claude::claude_client::{ClaudeClient, ClaudeOptions};
use crate::bedrock::models::claude::claude_request_message::{ChatOptions, ContentBlockDelta, ConversationRequest, Message, StreamResultData};
use crate::fetch_and_base64_encode_image;

async fn image_with_claude() {
    let claude_options = ClaudeOptions::new()
        .profile_name("bedrock")
        .region("us-west-2");

    let client = ClaudeClient::new(claude_options).await;

    let image_url = "./data/mario.png";
    let input_text = "What's in this image?".to_string();
    let image = fetch_and_base64_encode_image(image_url).await.unwrap().to_string();
    let mime_type = "image/png".to_string();

    let message = Message::new_user_message_with_image(&input_text, &image, &mime_type);

    let mut conversation_request = ConversationRequest::default();
    conversation_request.messages.push(message);

    let chat_options = ChatOptions::default()
        .with_temperature(0.7)
        .with_max_tokens(100);

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
    image_with_claude().await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_image_with_claude() {
        image_with_claude().await;
    }
}