
use std::io::Read;
use reqwest::Client;
use base64::{ encode };

use std::error::Error;
use std::fs::File;
use std::io::{ Read as CursorRead };
use url::Url;


pub async fn fetch_and_base64_encode_image(path: &str) -> Result<String, Box<dyn Error>> {
    // Check if the path is a valid URL
    if let Ok(url) = Url::parse(path) {
        // If it's a URL, download the image
        let client = Client::new();
        let response = client.get(url.as_str()).send().await?;

        if response.status().is_success() {
            let bytes = response.bytes().await?;
            let base64_string = encode(&bytes);
            Ok(base64_string)
        } else {
            Err(From::from("Failed to download the image for path: ".to_string() + path))
        }
    } else {
        // If it's not a URL, assume it's a local file path
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let base64_string = encode(&buffer);
        Ok(base64_string)
    }
}