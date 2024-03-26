use std::io::{self, Write};
use futures_util::StreamExt;
use hiramu::ollama::models::GenerateRequestBuilder;
use hiramu::ollama::ollama_client::OllamaClient;
use hiramu::util::fetch_and_base64_encode_image;

#[tokio::test]
async fn test_generate() {
    let client = OllamaClient::new("http://localhost:11434".to_string());
    println!("✅ Client created.");

    let request = GenerateRequestBuilder::new("mistral".to_string())
        .prompt("You are talking like a pirate: Explain why France is a great country in less than 40 words.".to_string())
        .build();

    let mut response_stream = client.generate(request).await;
    println!("✅ Request sent to the pirate model. 😂");

    while let Some(response_result) = response_stream.next().await {
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

    let mut response_stream = client.generate(request).await;
    println!("✅ Request sent to the llava model. 🌋");

    while let Some(response_result) = response_stream.next().await {
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

    let mut response_stream = client.generate(request).await;
    println!("✅ Request sent to the mistral model with custom options.");

    while let Some(response_result) = response_stream.next().await {
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