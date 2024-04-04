//! # Hiramu
//!
//! Hiramu is a Rust library that provides a high-level interface for interacting with various AI models and APIs, including Ollama and Bedrock. It simplifies the process of generating text, engaging in chat conversations, and working with different AI models.
//!
//! ## Overview
//!
//! The library consists of the following main components:
//!
//! - `ollama` module: Provides an interface for generating text and engaging in chat conversations using the Ollama API.
//! - `bedrock` module: Offers an interface for interacting with the Bedrock AI service, including generating text and engaging in chat conversations with models like Claude.
//! - `error` module: Defines the error handling mechanisms used throughout the library.
//! - `util` module: Contains utility functions for common tasks, such as encoding images to base64.
//!
//! ## Getting Started
//!
//! To start using Hiramu, add the following to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! hiramu = "0.1.x"
//! ```
//!
//! Then, import the necessary modules and types in your Rust code:
//!
//! ```rust
//! use hiramu::ollama::ollama_client::OllamaClient;
//! use hiramu::ollama::model::{GenerateRequest, GenerateRequestBuilder, GenerateResponse};
//! use hiramu::bedrock::bedrock_client::{BedrockClient, BedrockClientOptions};
//! use hiramu::bedrock::models::claude::claude_client::{ClaudeClient, ClaudeOptions};
//! use hiramu::bedrock::models::claude::claude_request_message::{ChatOptions, ConversationRequest, Message};
//! ```
//!
//! ## Examples
//!
//! ### Generating Text with Ollama
//!
//! ```rust
//! use hiramu::ollama::ollama_client::OllamaClient;
//! use hiramu::ollama::model::{GenerateRequest, GenerateRequestBuilder};
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = OllamaClient::new("http://localhost:11434".to_string());
//!     let request = GenerateRequestBuilder::new("mistral".to_string())
//!         .prompt("Once upon a time".to_string())
//!         .build();
//!
//!     let response_stream = client.generate(request).await.unwrap();
//!
//!     response_stream
//!         .try_for_each(|chunk| async move {
//!             println!("{}", chunk.response);
//!             Ok(())
//!         })
//!         .await
//!         .unwrap();
//! }
//! ```
//!
//! ### Chatting with Claude using Bedrock
//!
//! ```rust
//! use hiramu::bedrock::models::claude::claude_client::{ClaudeClient, ClaudeOptions};
//! use hiramu::bedrock::models::claude::claude_request_message::{ChatOptions, ConversationRequest, Message};
//!
//! #[tokio::main]
//! async fn main() {
//!     let claude_options = ClaudeOptions::new()
//!         .profile_name("bedrock")
//!         .region("us-west-2");
//!
//!     let client = ClaudeClient::new(claude_options).await;
//!
//!     let mut conversation_request = ConversationRequest::default();
//!     conversation_request
//!         .messages
//!         .push(Message::new_user_message("Hello, Claude!"));
//!
//!     let chat_options = ChatOptions::default()
//!         .with_temperature(0.7)
//!         .with_max_tokens(100);
//!
//!     let response_stream = client
//!         .chat_with_stream(&conversation_request, &chat_options)
//!         .await
//!         .unwrap();
//!
//!     response_stream
//!         .try_for_each(|chunk| async move {
//!             println!("{:?}", chunk);
//!             Ok(())
//!         })
//!         .await
//!         .unwrap();
//! }
//! ```
//!
//! For more examples and detailed usage instructions, please refer to the [examples](examples) directory and the documentation of individual modules.
//!
//! ## Error Handling
//!
//! The `error` module defines the `HiramuError` enum, which represents all possible errors that can occur within the library. This enum includes variants for HTTP errors, JSON parsing errors, I/O errors, UTF-8 conversion errors, invalid responses, API errors, and unknown errors.
//!
//! When an error occurs, the corresponding variant of `HiramuError` is returned, providing more details about the error. Developers can handle these errors using standard Rust error handling mechanisms, such as pattern matching or the `?` operator.
//!
//! ## Contributing
//!
//! Contributions to Hiramu are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request on the [GitHub repository](https://github.com/your/repository).
//!
//! To contribute to the project, follow these steps:
//!
//! 1. Fork the repository and create a new branch for your changes.
//! 2. Make your changes and ensure that all tests pass.
//! 3. Update the documentation if necessary.
//! 4. Submit a pull request with a detailed description of your changes.
//!
//! ## License
//!
//! Hiramu is licensed under the [MIT License](LICENSE).
//!
//! ## Acknowledgements
//!
//! Hiramu is built on top of the following libraries and APIs:
//!
//! - [Ollama](https://ollama.com/)
//! - [Bedrock](https://bedrock.com/)
//! - [reqwest](https://docs.rs/reqwest)
//! - [tokio](https://tokio.rs/)
//! - [serde](https://serde.rs/)
//!
//! We would like to express our gratitude to the developers and maintainers of these projects for their excellent work.
//!

pub mod ollama;
pub mod bedrock;
pub mod error;
pub mod util;

pub mod examples;

pub use error::HiramuError;
pub use util::fetch_and_base64_encode_image;