//! # ClaudeRequest
//!
//! The `ClaudeRequest` struct represents the request payload for the Claude model from Anthropic.
//! The request body is passed in the body field of a request to InvokeModel or InvokeModelWithResponseStream.
//!
//! ## Usage
//!
//! ```rust
//! use claude_request::ClaudeRequest;
//!
//! let request = ClaudeRequest::new("What is the capital of France?")
//!     .with_temperature(0.7)
//!     .with_top_p(0.9)
//!     .with_top_k(50)
//!     .with_max_tokens(100)
//!     .with_stop_sequences(vec!["<|endoftext|>", "\n"]);
//!
//! // Use the request to interact with the Claude model
//! // ...
//! ```
//!
//! ## Fields
//!
//! - `prompt`: The input prompt for the model.
//! - `temperature`: The temperature to use for the model's sampling.
//! - `top_p`: The top-p value to use for the model's sampling.
//! - `top_k`: The top-k value to use for the model's sampling.
//! - `max_tokens_to_sample`: The maximum number of tokens to generate.
//! - `stop_sequences`: A list of stop sequences to use for the model's generation.
//!
//! ## Methods
//!
//! - `new(prompt: &str)`: Creates a new `ClaudeRequest` instance with the given prompt.
//! - `with_temperature(self, temperature: f64)`: Sets the temperature for the request.
//! - `with_top_p(self, top_p: f64)`: Sets the top-p value for the request.
//! - `with_top_k(self, top_k: usize)`: Sets the top-k value for the request.
//! - `with_max_tokens(self, max_tokens: usize)`: Sets the maximum number of tokens to generate.
//! - `with_stop_sequences(self, stop_sequences: Vec<&str>)`: Sets the stop sequences for the request.

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ClaudeRequest {
    pub prompt: String,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub top_k: Option<usize>,
    pub max_tokens_to_sample: Option<usize>,
    pub stop_sequences: Option<Vec<String>>,
}

impl ClaudeRequest {
    /// Creates a new `ClaudeRequest` instance with the given prompt.
    pub fn new(prompt: &str) -> Self {
        ClaudeRequest {
            prompt: prompt.to_string(),
            temperature: None,
            top_p: None,
            top_k: None,
            max_tokens_to_sample: None,
            stop_sequences: None,
        }
    }

    /// Sets the temperature for the request.
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    /// Sets the top-p value for the request.
    pub fn with_top_p(mut self, top_p: f32) -> Self {
        self.top_p = Some(top_p);
        self
    }

    /// Sets the top-k value for the request.
    pub fn with_top_k(mut self, top_k: usize) -> Self {
        self.top_k = Some(top_k);
        self
    }

    /// Sets the maximum number of tokens to generate.
    pub fn with_max_tokens(mut self, max_tokens: usize) -> Self {
        self.max_tokens_to_sample = Some(max_tokens);
        self
    }

    /// Sets the stop sequences for the request.
    pub fn with_stop_sequences(mut self, stop_sequences: Vec<&str>) -> Self {
        self.stop_sequences = Some(stop_sequences.iter().map(|s| s.to_string()).collect());
        self
    }
}