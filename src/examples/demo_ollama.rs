use std::io::{self, Write};
use std::u32;

use futures::stream::TryStream;
use futures_util::TryStreamExt;

use crate::ollama::{ChatRequestBuilder, ChatResponse, Message, OllamaError};
use crate::ollama::{GenerateRequestBuilder, GenerateResponse, OllamaClient};

pub async fn chat_response_loop(max_loop: u32) {
    let client = OllamaClient::new("http://localhost:11434".to_string());

    let mut messages = Vec::new();
    let mut counter = 0;

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

        counter += 1;
        if counter >= max_loop {
            break;
        }
    }
}

pub async fn generate_response_loop(max_loop: usize) {
    let client = OllamaClient::new("http://localhost:11434".to_string());

    let mut counter = 0;
    loop {
        let input = prompt_input("\n> ").unwrap();
        let request = GenerateRequestBuilder::new("mistral".to_string())
            .prompt(input)
            .build();

        let response = client.generate(request).await.unwrap();

        print_generate_response(response).await.unwrap();

        counter += 1;
        if counter >= max_loop {
            break;
        }
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
    response: impl TryStream<Ok = ChatResponse, Error = OllamaError>,
    callback: F,
) -> Result<String, OllamaError>
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

pub async fn print_generate_response(
    response: impl TryStream<Ok = GenerateResponse, Error = OllamaError>,
) -> Result<(), OllamaError> {
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

// Create a test

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_chat_response_loop() {
        // chat_response_loop(1).await;
    }

    #[tokio::test]
    async fn test_generate_response_loop() {
        //      generate_response_loop(1).await;
    }
}
