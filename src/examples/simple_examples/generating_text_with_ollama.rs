use std::io::Write;

use futures::TryStreamExt;

use crate::ollama::ollama_client::OllamaClient;
use crate::ollama::model::GenerateRequestBuilder;

// #[allow(unused)]
pub async fn generating_text_with_ollama() {
    let client = OllamaClient::new("http://localhost:11434".to_string());
    let request = GenerateRequestBuilder::new("mistral".to_string())
        .prompt("Complete this story, less than 100 words: Once upon a time".to_string())
        .build();

    let response_stream = client.generate(request).await.unwrap();

    response_stream
        .try_for_each(|chunk| async move {
            print!("{}", chunk.response);
            std::io::stdout().flush()?;
            Ok(())
        })
        .await
        .unwrap();
}

// Main
#[tokio::main]
// no warnings
// #[allow(unused)]
pub async fn main() {
    generating_text_with_ollama().await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generating_text_with_ollama() {
        generating_text_with_ollama().await;
    }
}

