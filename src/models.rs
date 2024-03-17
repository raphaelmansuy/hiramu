use serde::{Deserialize, Serialize};

#[derive(Serialize,Debug,Deserialize)]
pub struct GenerateRequest {
    pub model: String,
    pub prompt: String,
}

#[derive(Deserialize,Debug,Serialize)]
pub struct GenerateResponse {
    pub model: String,
    pub created_at: String,
    pub response: String,
    pub done: bool,
}