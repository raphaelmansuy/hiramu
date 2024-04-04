use crate::ollama::{EmbeddingsRequestBuilder, OllamaClient};

async fn demo_ollama_embedding() -> Result<(), Box<dyn std::error::Error>> {
    let client = OllamaClient::new("http://localhost:11434".to_string());

    let prompt = "The quick brown fox jumps over the lazy dog.";

    let request = EmbeddingsRequestBuilder::new("nomic-embed-text".to_string(), prompt.to_string())
        .keep_alive("10m".to_string())
        .build();

    match client.embeddings(request).await {
        Ok(response) => {
            // Print embeddings dimensions
            println!("Embeddings dimensions: {:?}", response.embedding.len());
            println!("Embeddings: {:?}", response);
        }
        Err(error) => {
            eprintln!("Error: {:?}", error);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn main() {
        demo_ollama_embedding().await.unwrap();
    }
}
