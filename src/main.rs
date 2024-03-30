use futures::stream::TryStream; // Add this import statement
use futures_util::TryStreamExt;
use hiramu::ollama::models::ChatRequestBuilder;
use hiramu::ollama::models::ChatResponse;
use hiramu::ollama::models::GenerateRequestBuilder;
use hiramu::ollama::models::Message;
use hiramu::ollama::ollama_client::FetchStreamError;
use hiramu::ollama::ollama_client::OllamaClient;
use hiramu::GenerateResponse;
use std::io::{self, Write};
use tokio; // Add this import statement

#[tokio::main]
async fn main() {
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

        let response = print_and_get_chat_response(response_stream).await.unwrap();

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

async fn print_and_get_chat_response(
    response: impl TryStream<Ok = ChatResponse, Error = FetchStreamError>,
) -> Result<String, FetchStreamError> {
    let words = response
        .try_fold(String::new(), |mut f, chunk| async {
            let response = chunk.message.content;
            print!("{}", response);
            io::stdout().flush().unwrap();
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
