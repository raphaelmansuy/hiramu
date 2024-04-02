# Hiramu: A Foundational Layer for an AI Operating System with Agent Support

Hiramu is a Rust API that serves as a foundational layer for building an AI operating system with native support for AI agents. It provides a unified interface for interacting with various AI services, including Ollama, Bedrock, and Claude, enabling seamless integration and communication between different components of the AI system. Hiramu simplifies the process of making requests to these services and handling their responses, acting as a central hub for AI functionality.

The key feature of Hiramu will be its built-in support for AI agents. Agents are autonomous entities that can perform tasks, make decisions, and interact with other components of the AI system. Hiramu will provide a standardized way to define, manage, and deploy AI agents, making it easier to build complex AI applications that involve multiple agents working together.

With Hiramu, developers can focus on designing and implementing high-level agent behaviors and interactions, while relying on the robust and efficient infrastructure provided by this foundational layer. Hiramu abstracts away the low-level details of communication, coordination, and resource management, allowing developers to concentrate on the core logic of their AI agents.

By leveraging Hiramu as the foundation, developers can create AI systems that exhibit intelligent behavior, adapt to changing environments, and collaborate with other agents to achieve common goals. Hiramu's agent-centric approach enables the development of sophisticated AI applications across various domains, such as autonomous systems, robotics, intelligent assistants, and decision support systems.

In this early alpha release, Hiramu provides an API to access several AI foundation models and local models, including Ollama for local AI model serving, Bedrock for AWS AI platform integration, and Claude for advanced language models. The API offers a unified and developer-friendly interface to interact with these services, handle request and response serialization, support asynchronous request handling and streaming, and provide utility functions and example code to facilitate usage.

## Features

- Support for multiple AI services: Ollama, Bedrock, and Claude
- Easy-to-use client structs for each service
- Asynchronous request handling using Tokio and Futures
- Streaming support for real-time response processing
- Flexible request builders for customizing API requests
- Comprehensive error handling using custom error types
- Utility functions for common tasks like image encoding

## Installation

To use Hiramu in your Rust project, add the following to your `Cargo.toml` file:

```toml
[dependencies]
hiramu = "0.1.0"
```

Then, run `cargo build` to download and compile the dependencies.

## Usage

Here are some examples of how to use Hiramu in your Rust code:

### Ollama Client

```rust
use hiramu::ollama::ollama_client::OllamaClient;
use hiramu::ollama::model::{GenerateRequestBuilder, ChatRequestBuilder};

async fn main() {
    let client = OllamaClient::new("http://localhost:11434".to_string());

    // Generate request
    let generate_request = GenerateRequestBuilder::new("mistral".to_string())
        .prompt("Hello, how are you?")
        .build();
    let generate_response = client.generate(generate_request).await.unwrap();

    // Chat request
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
}
```

### Bedrock Client

```rust
use hiramu::bedrock::bedrock_client::{BedrockClient, BedrockClientOptions};
use hiramu::bedrock::model_info::{ModelInfo, ModelName};

async fn main() {
    let options = BedrockClientOptions::new()
        .profile_name("bedrock")
        .region("us-west-2");
    let client = BedrockClient::new(options).await;

    let model_id = ModelInfo::from_model_name(ModelName::AnthropicClaudeHaiku1x);
    let payload = serde_json::json!({
        "anthropic_version": "bedrock-2023-05-31",
        "max_tokens": 1000,
        "messages": [{
            "role": "user",
            "content": [{
                "type": "text",
                "text": "Hi. In a short paragraph, explain what you can do."
            }]
        }]
    });

    let response = client.generate_raw(model_id.to_string(), payload).await.unwrap();
}
```

### Claude Client

```rust
use hiramu::bedrock::models::claude::claude_client::{ClaudeClient, ClaudeOptions};
use hiramu::bedrock::models::claude::claude_request_message::{ChatOptions, ConversationRequest, Message};

async fn main() {
    let claude_options = ClaudeOptions::new()
        .profile_name("bedrock")
        .region("us-west-2");
    let client = ClaudeClient::new(claude_options).await;

    let mut conversation_request = ConversationRequest::default();
    conversation_request.messages.push(Message::new_user_message(
        "What is the capital of France?".to_owned(),
    ));

    let chat_options = ChatOptions::default()
        .with_temperature(0.5)
        .with_model_id("anthropic.claude-3-haiku-20240307-v1:0".to_string());

    let response = client.chat(&conversation_request, &chat_options).await.unwrap();
}
```

### Streaming Example

```rust
use futures::TryStreamExt;
use hiramu::bedrock::models::claude::claude_client::{ClaudeClient, ClaudeOptions};
use hiramu::bedrock::models::claude::claude_request_message::{ChatOptions, ConversationRequest, Message};

async fn main() {
    let claude_options = ClaudeOptions::new()
        .profile_name("bedrock")
        .region("us-west-2");
    let client = ClaudeClient::new(claude_options).await;

    let mut conversation_request = ConversationRequest::default();
    conversation_request.messages.push(Message::new_user_message(
        "What is the capital of France?".to_owned(),
    ));

    let chat_options = ChatOptions::default()
        .with_temperature(0.5)
        .with_model_id("anthropic.claude-3-haiku-20240307-v1:0".to_string());

    let response_stream = client.chat_with_stream(&conversation_request, &chat_options).await.unwrap();

    response_stream
        .try_for_each(|chunk| async move {
            let json_display = serde_json::to_string_pretty(&chunk).unwrap();
            println!("{:?}", json_display);
            Ok(())
        })
        .await
        .unwrap();
}
```

For more detailed usage examples, refer to the documentation and the `examples` directory in the source code.

## API Documentation

The API documentation is available at [https://docs.rs/hiramu](https://docs.rs/hiramu). It provides detailed information about the available structs, methods, and their usage.

## Examples

The `examples` directory in the source code contains various example scripts demonstrating how to use different features of Hiramu. You can run these examples using the following command:

```shell
cargo run --example <example_name>
```

Replace `<example_name>` with the name of the example you want to run.

## Contributing

Contributions to Hiramu are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request on the GitHub repository.

When contributing, please ensure that your code follows the existing coding style and conventions. Also, make sure to update the documentation and tests accordingly.

## License

Hiramu is released under the [MIT License](LICENSE).

## Acknowledgements

Hiramu is built upon the excellent work of the following libraries and frameworks:

- [Tokio](https://tokio.rs/) for asynchronous runtime and networking
- [Futures](https://docs.rs/futures/) for asynchronous programming abstractions
- [Reqwest](https://docs.rs/reqwest/) for making HTTP requests
- [Serde](https://serde.rs/) for serialization and deserialization of JSON data

We would like to express our gratitude to the developers and contributors of these projects.

## Contact

If you have any questions, suggestions, or feedback, please feel free to reach out to us at [contact@elitizon.com](mailto:contact@elitizon.com).

Happy coding with Hiramu!
