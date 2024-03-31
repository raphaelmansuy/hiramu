use std::io::{self, Write};

use futures::stream::TryStream;
use futures_util::TryStreamExt;
use tokio;

use hiramu::ollama::models::ChatRequestBuilder;
use hiramu::ollama::models::ChatResponse;
use hiramu::ollama::models::GenerateRequestBuilder;
use hiramu::ollama::models::Message;
use hiramu::ollama::ollama_client::FetchStreamError;
use hiramu::ollama::ollama_client::OllamaClient;

use hiramu::bedrock::bedrock_client::BedrockClient;
use hiramu::GenerateResponse;

async fn demo_generate_raw() {
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

    let result = BedrockClient::generate_raw(
        model_id.to_string(),
        profile_name.to_string(),
        region.to_string(),
        payload,
    )
    .await
    .unwrap();

    println!("{:?}", result);
}

async fn demo_generate_raw_stream() {
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

    let stream = BedrockClient::generate_raw_stream(
        model_id.to_string(),
        profile_name.to_string(),
        region.to_string(),
        payload,
    )
    .await;

    // consumme the stream and print the response
    stream
        .try_for_each(|chunk|  async move {
            print!("{:?}", chunk);
            // Flush the output to ensure the prompt is displayed.
            io::stdout().flush().unwrap();
            Ok(())
        })
        .await
        .unwrap();


}


#[tokio::main]
async fn main() {
    //demo_generate_raw().await;

    demo_generate_raw_stream().await;

    // generate_response_loop().await;
    chat_response_loop().await;
}

async fn chat_response_loop() {
    let client = OllamaClient::new("http://localhost:11434".to_string());

    let mut messages = Vec::new();

    loop {
        let input = prompt_input("\nUser: ").unwrap();
        messages.push(Message {
            role: "user".to_string(),
            content: input,
            images: vec![],
        });

        let request = ChatRequestBuilder::new("mistral".to_string())
            .messages(messages.clone())
            .build();

        let response_stream = client.chat(request).await.unwrap();

        let response = process_and_collect_chat_response(response_stream, |chunk| {
            print!("{}", chunk);
            io::stdout().flush().unwrap();
        })
        .await
        .unwrap();
        // get last response from the chat

        messages.push(Message {
            role: "assistant".to_string(),
            content: response,
            images: vec![],
        });
    }
}

async fn generate_response_loop() {
    let client = OllamaClient::new("http://localhost:11434".to_string());

    loop {
        let input = prompt_input("\n> ").unwrap();
        let request = GenerateRequestBuilder::new("mistral".to_string())
            .prompt(input)
            .build();

        let response = client.generate(request).await.unwrap();

        print_generate_response(response).await.unwrap();
    }
}

fn prompt_input(prompt: &str) -> Result<String, std::io::Error> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

async fn process_and_collect_chat_response<F>(
    response: impl TryStream<Ok = ChatResponse, Error = FetchStreamError>,
    callback: F,
) -> Result<String, FetchStreamError>
where
    F: Fn(&str) + Send + Sync + 'static,
{
    let words = response
        .try_fold(String::new(), |mut f, chunk| async {
            let response = chunk.message.content;
            callback(&response);
            f.push_str(&response);
            Ok(f)
        })
        .await
        .unwrap();

    Ok(words)
}

async fn print_generate_response(
    response: impl TryStream<Ok = GenerateResponse, Error = FetchStreamError>,
) -> Result<(), FetchStreamError> {
    response
        .try_for_each(|chunk| async {
            let response = chunk.response;
            print!("{}", response);
            // Flush the output to ensure the prompt is displayed.
            io::stdout().flush().unwrap();
            Ok(())
        })
        .await
}
