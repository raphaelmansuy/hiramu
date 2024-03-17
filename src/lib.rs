
/*
#[derive(Serialize, Deserialize, Debug)]
struct GenerateRequest {
    model: String,
    prompt: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GenerateResponse {
    pub model: String,
    pub created_at: String,
    pub response: String,
    pub done: bool,
}

pub struct Hiramu {
    pub client: Client,
    pub base_url: String,
}

impl Hiramu {
    pub fn new(base_url: &str) -> Self {
        Hiramu {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }
    pub async fn generate(&self, model: &str, prompt: &str) -> Result<Vec<GenerateResponse>, Box<dyn Error>> {
        let request = GenerateRequest {
            model: model.to_string(),
            prompt: prompt.to_string(),
        };

        let url = format!("{}/api/generate", self.base_url);
        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await?;

        // Inspect response
        println!("Response: {:?}", response);

        if response.status().is_success() {
            // Read the response body as text
            let text = response.text().await?;
            // Create a BufReader to read the text line by line
            let reader = BufReader::new(text.as_bytes());
            // Collect the JSONL lines into a vector of GenerateResponse
            let generate_response: Result<Vec<GenerateResponse>, _> = reader.lines()
            .map(|line| {
                match line {
                    Ok(line) => serde_json::from_str(&line),
                    Err(e) => Err(serde_json::Error::io(e)),
                }
            })
            .collect();
            match generate_response {
                Ok(responses) => {
                    println!("Gen Response: {:?}", responses);
                    Ok(responses)
                },
                Err(e) => Err(Box::new(e)),
            }
        } else {
            let error_text = response.text().await?;
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("API error: {}", error_text),
            )))
        }
    }
}*/


mod client;
mod error;
mod models;

pub use client::Hiramu;
pub use error::HiramuError;
pub use models::{GenerateRequest, GenerateResponse};

#[cfg(test)]
mod tests;
