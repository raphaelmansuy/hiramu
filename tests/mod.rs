use std::io::{ self, Write };
use hiramu::ollama::OllamaClientBuilder;
use hiramu::ollama::OllamaClient;
use hiramu::GenerateRequestBuilder;
use hiramu::chat::Chat;
use hiramu::LLMClient;
use futures_util::StreamExt;
use std::pin::Pin; // For pinning the stream in the return type
use std::io::Read;
use reqwest::Client;
use base64::{ encode, Engine as _ };

use std::error::Error;
use std::fs::File;
use std::io::{ Cursor, Read as CursorRead };
use url::Url;

use hiramu::util::fetch_and_base64_encode_image;

#[tokio::test]
async fn test_generate() {
    let client = OllamaClientBuilder::new().url("http://localhost:11434").build();


    println!("✅ Client created.");
    let request = GenerateRequestBuilder::new(
        "mistral".to_string(),
        "You are talking like a pirate: Explain why France is great country, less than 40 words ?".to_string()
    ).build();

    let mut response_stream = client.generate(request);
    println!("✅ Request sent to the pirate model. 😂");

    while let Some(response_result) = response_stream.next().await {
        match response_result {
            Ok(response) => {
                // print the response, we need to flush the output to see it immediately without a newline
                print!("{}", response.response);
                // Flush the output to see it immediately without a newline
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

    // Download and base64 encode the image
    let image = fetch_and_base64_encode_image(image_url).await.unwrap();

    println!("✅ Image path: {} downloaded and transformed.", image_url);


    let client = OllamaClientBuilder::new().url("http://localhost:11434").build();
    let request = GenerateRequestBuilder::new(
        "llava".to_string(),
        "Describe this image in a few words".to_string()
    )
        .images(vec![image])
        .build();


    let mut response_stream = client.generate(request);

    println!("✅ Request sent to the llava model. 🌋");

    while let Some(response_result) = response_stream.next().await {
        match response_result {
            Ok(response) => {
                // print the response, we need to flush the output to see it immediately without a newline
                print!("{}", response.response);
                // Flush the output to see it immediately without a newline
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
