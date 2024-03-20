
use std::io::Read;

use base64::{engine::general_purpose, Engine as _};
use reqwest::Response;

use std::error::Error;
use std::fs::File;
use url::Url;


pub async fn fetch_and_base64_encode_image(path: &str) -> Result<String, Box<dyn Error>> {
    // Check if the path is a valid URL
    if let Ok(url) = Url::parse(path) {
        // If it's a URL, download the image
        let client = reqwest::Client::new();
        let response = client.get(url.as_str()).send().await?;
        let base64_string = encode_response_to_base64(response).await?;
        Ok(base64_string)
    } else {
        // If it's not a URL, assume it's a local file path
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let base64_string = general_purpose::STANDARD.encode(buffer);
        Ok(base64_string)
    }
}

async fn encode_response_to_base64(response: Response) -> Result<String, Box<dyn Error>> {
    if response.status().is_success() {
        let bytes = response.bytes().await?;
        let base64_string = general_purpose::STANDARD.encode(bytes);
        Ok(base64_string)
    } else {
        Err(From::from("Failed to download the image"))
    }
}