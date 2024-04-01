use crate::bedrock::bedrock_client::BedrockClient;
use futures::TryStreamExt;
use std::io;
use std::io::Write;

pub async fn demo_generate_raw_stream() {
    let model_id = "anthropic.claude-3-haiku-20240307-v1:0";
    let profile_name = "bedrock";
    let region = "us-west-2";

    let prompt = "Hi. In a short paragraph, explain what you can do.";

    let payload = serde_json::json!({
        "anthropic_version": "bedrock-2023-05-31",
        "max_tokens": 1000,
        "messages": [{
            "role": "user",
            "content": [{
                "type": "text",
                "text": prompt
            }]
        }]
    });

    let client = BedrockClient::new();

    let stream = client
        .generate_raw_stream(
            model_id.to_string(),
            payload,
            Some(profile_name.to_string()),
            Some(region.to_string()),
        )
        .await;

    // consumme the stream and print the response
    stream
        .try_for_each(|chunk| async move {
            println!("{:?}", chunk);
            // Flush the output to ensure the prompt is displayed.
            io::stdout().flush().unwrap();
            Ok(())
        })
        .await
        .unwrap();
}


// Write a test

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_demo_generate_raw_stream() {
        demo_generate_raw_stream().await;
    }
}
