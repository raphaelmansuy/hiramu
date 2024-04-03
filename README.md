# Hiramu: An AI Toolbox for Simplified Access to AI Models


Hiramu is an open-source AI toolbox written in Rust that simplifies access to various AI models. It provides a user-friendly and efficient way to interact with AI models, making it easier for developers to integrate AI capabilities into their applications.

## Features

- **Simplified AI Model Access**: Hiramu offers a streamlined interface for accessing and utilizing different AI models, abstracting away the complexities of individual model APIs.
- **Extensible Architecture**: The toolbox is designed with extensibility in mind, allowing developers to easily add support for new AI models and providers.
- **Asynchronous and Efficient**: Leveraging Rust's async/await capabilities, Hiramu enables efficient and non-blocking communication with AI models, optimizing performance.
- **Comprehensive Error Handling**: Hiramu provides a robust error handling mechanism, ensuring graceful error management and informative error messages.
- **Stream-based Responses**: The toolbox supports stream-based responses, enabling real-time processing of AI-generated content.
- **Flexible Configuration**: Hiramu offers flexible configuration options, allowing developers to customize the behavior of AI models based on their specific requirements.

## Supported AI Models

Hiramu currently supports the following AI models:

- **Ollama**: A powerful language model for natural language processing tasks.
- **Bedrock**: A versatile AI platform offering various models for text generation, image processing, and more.
  - **Claude**: A state-of-the-art conversational AI model within the Bedrock ecosystem.

We are continuously expanding the range of supported AI models to provide developers with a diverse set of options.

## Getting Started

To get started with Hiramu, follow these steps:

1. Install Rust by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

2. Add Hiramu as a dependency to your Rust project by adding the following line to your `Cargo.toml` file:

   ```toml
   [dependencies]
   hiramu = "0.1.0"
   ```

3. Import the necessary modules and start using Hiramu in your Rust code:

   ```rust
   use hiramu::ollama::OllamaClient;
   use hiramu::bedrock::models::claude::ClaudeClient;

   async fn main() {
       // Create an instance of the OllamaClient
       let ollama_client = OllamaClient::new("http://localhost:11434".to_string());

       // Create an instance of the ClaudeClient
       let claude_options = ClaudeOptions::new()
           .profile_name("bedrock")
           .region("us-west-2");
       let claude_client = ClaudeClient::new(claude_options).await;

       // Use the clients to interact with the AI models
       // ...
   }
   ```

## Examples

Here are some examples demonstrating how to use Hiramu with different AI models:

### Ollama

```rust
use hiramu::ollama::OllamaClient;
use hiramu::ollama::model::{GenerateRequestBuilder, ChatRequestBuilder, Message};

async fn main() {
    let client = OllamaClient::new("http://localhost:11434".to_string());

    // Generate text using Ollama
    let generate_request = GenerateRequestBuilder::new("mistral".to_string())
        .prompt("Hello, how are you?".to_string())
        .build();
    let generate_response = client.generate(generate_request).await.unwrap();
    generate_response.try_for_each(|chunk| async move {
        println!("{}", chunk.response);
        Ok(())
    }).await.unwrap();

    // Chat with Ollama
    let chat_request = ChatRequestBuilder::new("mistral".to_string())
        .messages(vec![
            Message {
                role: "user".to_string(),
                content: "Hello, how are you?".to_string(),
                images: vec![],
            },
        ])
        .build();
    let chat_response = client.chat(chat_request).await.unwrap();
    chat_response.try_for_each(|chunk| async move {
        println!("{}", chunk.message.content);
        Ok(())
    }).await.unwrap();
}
```

### Bedrock - Claude

```rust
use hiramu::bedrock::models::claude::claude_client::{ClaudeClient, ClaudeOptions};
use hiramu::bedrock::models::claude::claude_request_message::{ConversationRequest, Message, ChatOptions};

async fn main() {
    let claude_options = ClaudeOptions::new()
        .profile_name("bedrock")
        .region("us-west-2");
    let client = ClaudeClient::new(claude_options).await;

    // Chat with Claude
    let mut conversation_request = ConversationRequest::default();
    conversation_request.messages.push(Message::new_user_message(
        "What is the capital of France?".to_owned(),
    ));
    let chat_options = ChatOptions::default()
        .with_temperature(0.5);
    let response = client.chat(&conversation_request, &chat_options).await.unwrap();
    println!("{:?}", response);

    // Chat with Claude using a stream
    let response_stream = client.chat_with_stream(&conversation_request, &chat_options).await.unwrap();
    response_stream.try_for_each(|chunk| async move {
        println!("{:?}", chunk);
        Ok(())
    }).await.unwrap();
}
```



## Contributing

We welcome contributions from the open-source community to enhance Hiramu and expand its capabilities. To contribute, please follow these steps:

1. Fork the repository on GitHub.
2. Create a new branch for your feature or bug fix.
3. Make your changes and ensure that the code passes all tests.
4. Submit a pull request describing your changes.



## License

Hiramu is released under the [MIT License](LICENSE.md).

## Contact

If you have any questions, suggestions, or feedback, please feel free to reach out to us:

- Email: contact@elitizon.com


We appreciate your interest in Hiramu and look forward to your contributions and feedback!
