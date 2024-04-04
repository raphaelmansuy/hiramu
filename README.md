# Hiramu

Hiramu is a powerful and flexible Rust library that provides a high-level interface for interacting with various AI models and APIs, including Ollama and Bedrock. It simplifies the process of generating text, engaging in chat conversations, and working with different AI models.

## Features

- Easy-to-use interfaces for generating text and engaging in chat conversations with AI models
- Support for Ollama and Bedrock AI services
- Asynchronous and streaming responses for efficient handling of large outputs
- Customizable options for fine-tuning the behavior of AI models
- Comprehensive error handling and informative error messages
- Well-documented code with examples and explanations


## Getting Started

To start using Hiramu in your Rust project, add the following to your `Cargo.toml` file:

```toml
[dependencies]
hiramu = "0.1.X"
```

Then, import the necessary modules and types in your Rust code:

```rust
use hiramu::ollama::ollama_client::OllamaClient;
use hiramu::ollama::model::{GenerateRequest, GenerateRequestBuilder, GenerateResponse};
use hiramu::bedrock::bedrock_client::{BedrockClient, BedrockClientOptions};
use hiramu::bedrock::models::claude::claude_client::{ClaudeClient, ClaudeOptions};
use hiramu::bedrock::models::claude::claude_request_message::{ChatOptions, ConversationRequest, Message};
```

## Examples

### Generating Text with Ollama

```rust
use hiramu::ollama::ollama_client::OllamaClient;
use hiramu::ollama::model::{GenerateRequest, GenerateRequestBuilder};

#[tokio::main]
async fn main() {
    let client = OllamaClient::new("http://localhost:11434".to_string());
    let request = GenerateRequestBuilder::new("mistral".to_string())
        .prompt("Once upon a time".to_string())
        .build();

    let response_stream = client.generate(request).await.unwrap();

    response_stream
        .try_for_each(|chunk| async move {
            println!("{}", chunk.response);
            Ok(())
        })
        .await
        .unwrap();
}
```

### Chatting with Claude using Bedrock

```rust
use hiramu::bedrock::models::claude::claude_client::{ClaudeClient, ClaudeOptions};
use hiramu::bedrock::models::claude::claude_request_message::{ChatOptions, ConversationRequest, Message};

#[tokio::main]
async fn main() {
    let claude_options = ClaudeOptions::new()
        .profile_name("bedrock")
        .region("us-west-2");

    let client = ClaudeClient::new(claude_options).await;

    let mut conversation_request = ConversationRequest::default();
    conversation_request
        .messages
        .push(Message::new_user_message("Hello, Claude!"));

    let chat_options = ChatOptions::default()
        .with_temperature(0.7)
        .with_max_tokens(100);

    let response_stream = client
        .chat_with_stream(&conversation_request, &chat_options)
        .await
        .unwrap();

    response_stream
        .try_for_each(|chunk| async move {
            println!("{:?}", chunk);
            Ok(())
        })
        .await
        .unwrap();
}
```

### Sending Images with Claude

```rust
use hiramu::bedrock::models::claude::claude_client::{ClaudeClient, ClaudeOptions};
use hiramu::bedrock::models::claude::claude_request_message::{ChatOptions, ConversationRequest, Message};
use hiramu::fetch_and_base64_encode_image;

#[tokio::main]
async fn main() {
    let claude_options = ClaudeOptions::new()
        .profile_name("bedrock")
        .region("us-west-2");

    let client = ClaudeClient::new(claude_options).await;

    let image_url = "./data/mario.png";
    let input_text = "What's in this image?".to_string();
    let image = fetch_and_base64_encode_image(image_url).await.unwrap().to_string();
    let mime_type = "image/png".to_string();

    let message = Message::new_user_message_with_image(&input_text, &image, &mime_type);

    let mut conversation_request = ConversationRequest::default();
    conversation_request.messages.push(message);

    let chat_options = ChatOptions::default()
        .with_temperature(0.7)
        .with_max_tokens(100);

    let response_stream = client
        .chat_with_stream(&conversation_request, &chat_options)
        .await
        .unwrap();

    response_stream
        .try_for_each(|chunk| async move {
            println!("{:?}", chunk);
            Ok(())
        })
        .await
        .unwrap();
}
```

### Using the Raw Bedrock API

#### Generating a Raw Response

```rust
use hiramu::bedrock::bedrock_client::{BedrockClient, BedrockClientOptions};
use hiramu::bedrock::model_info::{ModelInfo, ModelName};

#[tokio::main]
async fn main() {
    let model_id = ModelInfo::from_model_name(ModelName::AnthropicClaudeHaiku1x);
    let profile_name = "bedrock";
    let region = "us-west-2";

    let prompt = "Hi. In a short paragraph, explain what you can do.";

    let payload = serde_json::json!({
        "anthropic_version": "bedrock-2023-05-31",
        "max_tokens": 1000,
        "messages": [{
            "role": "user",
            "content": [{
                "type": "text",
                "text": prompt
            }]
        }]
    });

    let options = BedrockClientOptions::new()
        .profile_name(profile_name)
        .region(region);

    let client = BedrockClient::new(options).await;

    let result = client
        .generate_raw(model_id.to_string(), payload)
        .await
        .unwrap();

    println!("{:?}", result);
}
```

#### Generating a Raw Stream Response

```rust
use futures::TryStreamExt;
use hiramu::bedrock::bedrock_client::{BedrockClient, BedrockClientOptions};
use hiramu::bedrock::model_info::{ModelInfo, ModelName};

#[tokio::main]
async fn main() {
    let model_id = ModelInfo::from_model_name(ModelName::AnthropicClaudeHaiku1x);
    let profile_name = "bedrock";
    let region = "us-west-2";

    let prompt = "Hi. In a short paragraph, explain what you can do.";

    let payload = serde_json::json!({
        "anthropic_version": "bedrock-2023-05-31",
        "max_tokens": 1000,
        "messages": [{
            "role": "user",
            "content": [{
                "type": "text",
                "text": prompt
            }]
        }]
    });

    let options = BedrockClientOptions::new()
        .profile_name(profile_name)
        .region(region);

    let client = BedrockClient::new(options).await;

    let stream = client
        .generate_raw_stream(model_id.to_string(), payload)
        .await
        .unwrap();

    stream
        .try_for_each(|chunk| async move {
            println!("{:?}", chunk);
            Ok(())
        })
        .await
        .unwrap();
}
```

Here's a paragraph explaining how to use Embeddings in the Ollama Rust library for a README.md file:

## Embeddings

The Ollama library provides functionality to generate embeddings for a given text prompt. Embeddings are dense vector representations of text that capture semantic meaning and can be used for various downstream tasks such as semantic search, clustering, and classification. To generate embeddings, you can use the `OllamaClient::embeddings` method. First, create an instance of `EmbeddingsRequestBuilder` by providing the model name and the text prompt. Optionally, you can specify additional options and a keep-alive duration. Then, call the `build` method to create an `EmbeddingsRequest` and pass it to the `embeddings` method of the `OllamaClient`. The method returns an `EmbeddingsResponse` containing the generated embedding as a vector of floating-point values. Here's an example:

```rust
use ollama::{OllamaClient, EmbeddingsRequestBuilder};

let client = OllamaClient::new("http://localhost:11434".to_string());
let request = EmbeddingsRequestBuilder::new(
    "nomic-embed-text".to_string(),
    "Here is an article about llamas...".to_string(),
)
.options(serde_json::json!({ "temperature": 0.8 }))
.keep_alive("10m".to_string())
.build();

let response = client.embeddings(request).await.unwrap();
println!("Embeddings: {:?}", response.embedding);
```

This code snippet demonstrates how to create an `EmbeddingsRequestBuilder`, set the model name, prompt, options, and keep-alive duration, and then build the request. The `embeddings` method is called with the request, and the resulting `EmbeddingsResponse` contains the generated embedding.




## Contributing

Contributions to Hiramu are welcome! If you encounter any issues, have suggestions for improvements, or want to add new features, please open an issue or submit a pull request on the [GitHub repository](https://github.com/raphaelmansuy/hiramu).

To contribute to the project, follow these steps:

1. Fork the repository and create a new branch for your changes.
2. Make your modifications and ensure that the code compiles successfully.
3. Write tests to cover your changes and ensure that all existing tests pass.
4. Update the documentation, including the README and API docs, if necessary.
5. Submit a pull request with a clear description of your changes and the problem they solve.

## License

Hiramu is licensed under the [MIT License].

## Acknowledgements

Hiramu is built on top of the following libraries and APIs:

- [Ollama](https://ollama.com/)
- [reqwest](https://docs.rs/reqwest)
- [tokio](https://tokio.rs/)
- [serde](https://serde.rs/)

We would like to express our gratitude to the developers and maintainers of these projects for their excellent work and contributions to the Rust ecosystem.
