use hiramu::HiramuClient;
use hiramu::Chat;
use futures_util::stream::StreamExt; // Needed for .next() and other stream combinators
use tokio;
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    let client = HiramuClient::new("http://localhost:11434".to_string());
    let mut chat = Chat::new(client, "The assistant will act like a pirate".to_string());

    loop {
        let input = prompt_input("\n> ").unwrap();

        // Since add_message now returns a Stream, we need to consume it
        let mut response_stream = chat.add_message(input);

        // Consume the stream
        while let Some(response_result) = response_stream.next().await {
            match response_result {
                Ok(response) => {
                    // print the response, we need to flush the output to see it immediately without a newline
                    print!("{}", response.response);
                    // Flush the output to see it immediately without a newline
                    io::stdout().flush().unwrap();

                    
                    // Break if the response is marked as done, or continue processing
                    if response.done {
                        break;
                    }
                },
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                    break;
                }
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