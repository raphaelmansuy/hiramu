use futures_util::stream::StreamExt;
use hiramu::ollama::models::GenerateRequestBuilder;
use hiramu::ollama::ollama_client::OllamaClient;
use std::io::{self, Write};
use tokio;

#[tokio::main]
async fn main() {
    let client = OllamaClient::new("http://localhost:11434".to_string());

    loop {
        let input = prompt_input("\n> ").unwrap();
        let request = GenerateRequestBuilder::new("mistral".to_string())
            .prompt(input)
            .build();

        match client.generate(request).await {
            Ok(response_stream) => {
                let mut pinned_stream = Box::pin(response_stream);
                while let Some(response_result) = pinned_stream.next().await {
                    match response_result {
                        Ok(response) => {
                            print!("{}", response.response);
                            io::stdout().flush().unwrap();
                            if response.done {
                                break;
                            }
                        }
                        Err(e) => {
                            eprintln!("Error: {:?}", e);
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
            }
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
