use hiramu::ollama::models::GenerateRequestBuilder;
use hiramu::ollama::ollama_client::OllamaClient;
use std::io::{self, Write};
use tokio;
use futures_util::TryStreamExt;

#[tokio::main]
async fn main() {
    let client = OllamaClient::new("http://localhost:11434".to_string());

    loop {
        let input = prompt_input("\n> ").unwrap();
        let request = GenerateRequestBuilder::new("mistral".to_string())
            .prompt(input)
            .build();

        let response = client.generate(request).await.unwrap();


        response
            .try_for_each(|chunk| async {
                let response = chunk.response;
                print!("{}", response);
                // Flush the output to ensure the prompt is displayed.
                io::stdout().flush().unwrap();
                Ok(())
            })
            .await
            .unwrap();        

    }
}

fn prompt_input(prompt: &str) -> Result<String, std::io::Error> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}
