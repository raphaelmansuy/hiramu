use futures::TryStreamExt;

use crate::{bedrock::models::claude::{claude_client::{ClaudeClient, ClaudeOptions}, claude_request_message::{ChatOptions, ConversationRequest, Message}}, fetch_and_base64_encode_image};


async fn demo_claude_multimedia() {
    let claude_options = ClaudeOptions::new().profile_name("bedrock").region("us-west-2");

    let client = ClaudeClient::new(claude_options).await;

    let image_url = "./data/mario.png";

    let mut conversation_request = ConversationRequest::default();

    let input_text = "What's in this image?".to_string(); // Convert a string literal to a String object
    let image = fetch_and_base64_encode_image(image_url).await.unwrap().to_string(); // Example base64 encoded image data
    let mime_type = "image/png".to_string(); // MIME type for the image

    // Now, you can call the function with the correct types
    let message = Message::new_user_message_with_image(&input_text, &image, &mime_type);

    conversation_request.messages.push(message);

    let chat_options = ChatOptions::default().with_temperature(0.7).with_max_tokens(100);

    // display the conversation request, JSON pretty print
    println!("{}", serde_json::to_string_pretty(&conversation_request).unwrap());

    let response_stream = client
        .chat_with_stream(&conversation_request, &chat_options).await
        .unwrap();

    response_stream
        .try_for_each(|chunk| async move {
            println!("{:?}", chunk);
            Ok(())
        }).await
        .unwrap();
}

// Test
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_demo_claude_multimedia() {
        demo_claude_multimedia().await;
    }
}
