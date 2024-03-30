use futures_util::stream::TryStream; 
use futures_util::stream::TryStreamExt;
use hiramu::ollama::models::GenerateRequestBuilder;
use hiramu::ollama::models::GenerateResponse; 
use hiramu::ollama::ollama_client::FetchStreamError;
use hiramu::ollama::ollama_client::OllamaClient;
use hiramu::util::fetch_and_base64_encode_image;
use std::io::{self, Write}; 

async fn display_response(
    response_stream: impl TryStream<Ok = GenerateResponse, Error = FetchStreamError>,
) {
    response_stream
        .try_for_each(|response| async {
            let response = response.response;
            print!("{}", response);
            io::stdout().flush().unwrap();
            Ok(())
        })
        .await
        .unwrap();
}

#[tokio::test]
async fn test_generate() {
    let client = OllamaClient::new("http://localhost:11434".to_string());
    println!("✅ Client created.");

    let request = GenerateRequestBuilder::new("mistral".to_string())
        .prompt("You are talking like a pirate: Explain why France is a great country in less than 40 words.".to_string())
        .build();

    let response_stream = client.generate(request).await.unwrap();
    println!("✅ Request sent to the pirate model. 😂");

    display_response(response_stream).await;
}

#[tokio::test]
async fn test_generate_with_image() {
    let image_url = "./tests/data/mario.png";
    let image = fetch_and_base64_encode_image(image_url).await.unwrap();
    println!("✅ Image path: {} downloaded and transformed.", image_url);

    let client = OllamaClient::new("http://localhost:11434".to_string());
    let request = GenerateRequestBuilder::new("llava".to_string())
        .prompt("Describe this image in a few words.".to_string())
        .images(vec![image])
        .build();

    let response_stream = client.generate(request).await.unwrap();
    println!("✅ Request sent to the llava model. 🌋");

    display_response(response_stream).await;
}

#[tokio::test]
async fn test_generate_with_options() {
    let client = OllamaClient::new("http://localhost:11434".to_string());
    println!("✅ Client created.");

    let options = serde_json::json!({
        "temperature": 0.8,
        "top_p": 0.9,
        "num_predict": 50
    });

    let request = GenerateRequestBuilder::new("mistral".to_string())
        .prompt("Explain the concept of blockchain in simple terms.".to_string())
        .options(options)
        .build();

    let response_stream = client.generate(request).await.unwrap();
    println!("✅ Request sent to the mistral model with custom options.");

    display_response(response_stream).await;
}
