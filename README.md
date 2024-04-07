# Hiramu

Hiramu is a powerful and flexible Rust library that provides a high-level interface for interacting with various AI models and APIs, including Ollama and AWS Bedrock. 

It simplifies the process of generating text, engaging in chat conversations, and working with different AI models.

## Features

- Easy-to-use interfaces for generating text and engaging in chat conversations with AI models
- Support for Ollama and Bedrock AI services
- Convenient interface for Claude and Mistral for AWS Bedrock
- Asynchronous and streaming responses for efficient handling of large outputs
- Customizable options for fine-tuning the behavior of AI models
- Comprehensive error handling and informative error messages
- Well-documented code with examples and explanations

## Getting Started

To start using Hiramu in your Rust project, add the following to your `Cargo.toml` file:

```toml
[dependencies]
hiramu = "0.1.8"
```

## Examples

### Generating Text with Mistral

```rust
use hiramu::bedrock::model_info::{ModelInfo, ModelName};
use hiramu::bedrock::models::mistral::mistral_client::{MistralClient, MistralOptions};
use hiramu::bedrock::models::mistral::mistral_request_message::MistralRequestBuilder;

async fn generating_text_with_mistral() {
    let mistral_options = MistralOptions::new()
        .profile_name("bedrock")
        .region("us-west-2");

    let client = MistralClient::new(mistral_options).await;

    let request =
        MistralRequestBuilder::new("<s>[INST] What is the capital of France?[/INST]".to_string())
            .max_tokens(200)
            .temperature(0.8)
            .build();

    let model_id = ModelInfo::from_model_name(ModelName::MistralMixtral8X7BInstruct0x);
    let response = client.generate(model_id, &request).await.unwrap();

    println!("Response: {:?}", response.outputs[0].text);
}

```

### Streaming Text Generation with Mistral

```rust
use futures::stream::StreamExt;
use hiramu::bedrock::models::mistral::mistral_client::{MistralClient, MistralOptions};
use hiramu::bedrock::models::mistral::mistral_request_message::MistralRequestBuilder;
use hiramu::bedrock::model_info::{ModelInfo, ModelName};

pub async fn generating_text_with_mistral() {
    let mistral_options = MistralOptions::new()
        .profile_name("bedrock")
        .region("us-west-2");

    let client = MistralClient::new(mistral_options).await;

    let request = MistralRequestBuilder::new("<s>[INST] What is the capital of France?[/INST]".to_string())
        .max_tokens(200)
        .temperature(0.8)
        .build();

    let model_id = ModelInfo::from_model_name(ModelName::MistralMixtral8X7BInstruct0x);
    let mut stream = client.generate_with_stream(model_id, &request).await.unwrap();

    while let Some(result) = stream.next().await {
        match result {
            Ok(response) => {
                println!("Response: {:?}", response.outputs[0].text);
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
            }
        }
    }
}
```

### Generating Text with Ollama

```rust
use std::io::Write;

use futures::TryStreamExt;

use hiramu::ollama::ollama_client::OllamaClient;
use hiramu::ollama::model::{GenerateRequestBuilder};

async fn generating_text_with_ollama() {
    let client = OllamaClient::new("http://localhost:11434".to_string());
    let request = GenerateRequestBuilder::new("mistral".to_string())
        .prompt("Once upon a time".to_string())
        .build();

    let response_stream = client.generate(request).await.unwrap();

    response_stream
        .try_for_each(|chunk| async move {
            print!("{}", chunk.response);
            std::io::stdout().flush()?;
            Ok(())
        })
        .await
        .unwrap();
}
```

### Chatting with Claude using Bedrock

```rust
use std::io::Write;

use futures::TryStreamExt;

use hiramu::bedrock::model_info::{ModelInfo, ModelName};
use hiramu::bedrock::models::claude::claude_client::{ClaudeClient, ClaudeOptions};
use hiramu::bedrock::models::claude::claude_request_message::{
    ChatOptions, ContentBlockDelta, ConversationRequest, Message, StreamResultData,
};

pub async fn chat_with_claude() {
    let claude_options = ClaudeOptions::new()
        .profile_name("bedrock")
        .region("us-west-2");

    let client = ClaudeClient::new(claude_options).await;

    let mut conversation_request = ConversationRequest::default();
    conversation_request
        .messages
        .push(Message::new_user_message("Hello, Claude!".to_owned()));

    let chat_options = ChatOptions::default()
        .with_temperature(0.7)
        .with_max_tokens(100)
        .with_model_id(ModelInfo::from_model_name(
            ModelName::AnthropicClaudeHaiku1x,
        ));

    let response_stream = client
        .chat_with_stream(&conversation_request, &chat_options)
        .await
        .unwrap();

    response_stream
        .try_for_each(|chunk| async move {
            match chunk {
                StreamResultData::ContentBlockStart(..) => {
                    println!("\n------------------------------");
                }
                StreamResultData::ContentBlockStop(..) => {
                    println!("\n------------------------------");
                }
                StreamResultData::ContentBlockDelta(ContentBlockDelta { delta, .. }) => {
                    print!("{}", delta.text);
                    std::io::stdout().flush().unwrap();
                }
                _ => {}
            }
            Ok(())
        })
        .await
        .unwrap();
}
```

### Working with Images with Claude

```rust
use std::io::Write;

use futures::TryStreamExt;

use hiramu::bedrock::models::claude::claude_client::{ClaudeClient, ClaudeOptions};
use hiramu::bedrock::models::claude::claude_request_message::{ChatOptions, ContentBlockDelta, ConversationRequest, Message, StreamResultData};
use hiramu::fetch_and_base64_encode_image;

async fn image_with_claude() {
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
            match chunk {
                StreamResultData::ContentBlockStart(..) => {
                    println!("\n------------------------------");
                }
                StreamResultData::ContentBlockStop(..) => {
                    println!("\n------------------------------");
                }

                StreamResultData::ContentBlockDelta(ContentBlockDelta { delta, .. }) => {
                    print!("{}", delta.text);
                    std::io::stdout().flush().unwrap();
                }
                _ => {}
            }
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

## Using Embeddings with Ollama

```rust
use hiramu::ollama::{EmbeddingsRequestBuilder, OllamaClient};

pub async fn demo_ollama_embedding() -> Result<(), Box<dyn std::error::Error>> {
    let client = OllamaClient::new("http://localhost:11434".to_string());

    let prompt = "The quick brown fox jumps over the lazy dog.";

    let request = EmbeddingsRequestBuilder::new("nomic-embed-text".to_string(), prompt.to_string())
        .keep_alive("10m".to_string())
        .build();

    match client.embeddings(request).await {
        Ok(response) => {
            // Print embeddings dimensions
            println!("Embeddings dimensions: {:?}", response.embedding.len());
            println!("Embeddings: {:?}", response);
        }
        Err(error) => {
            eprintln!("Error: {:?}", error);
        }
    }

    Ok(())
}
```

## Examples

Here is a table with a description for each example:

| Example | Path | Description |
|---------|------|--------------|
| `demo_ollama` | [src/examples/demo_ollama.rs](src/examples/demo_ollama.rs) | A simple example that demonstrates how to use the Ollama API to generate responses to chat messages. |
| `demo_bedrock_raw_generate` | [src/examples/demo_bedrock_raw_generate.rs](src/examples/demo_bedrock_raw_generate.rs) | Demonstrates how to generate a raw response from the Bedrock service using the `generate_raw` method. |
| `demo_bedrock_raw_stream` | [src/examples/demo_bedrock_raw_stream.rs](src/examples/demo_bedrock_raw_stream.rs) | Demonstrates how to generate a raw stream of responses from the Bedrock service using the `generate_raw_stream` method. |
| `demo_bedrock_raw_mistral` | [src/examples/demo_bedrock_raw_mistral.rs](src/examples/demo_bedrock_raw_mistral.rs) | Demonstrates how to generate a raw stream of responses from the Mistral model in the Bedrock service. |
| `demo_claude_chat` | [src/examples/demo_claude_chat.rs](src/examples/demo_claude_chat.rs) | Demonstrates how to use the Claude model in the Bedrock service to generate a chat response. |
| `demo_claude_chat_stream` | [src/examples/demo_claude_chat_stream.rs](src/examples/demo_claude_chat_stream.rs) | Demonstrates how to use the Claude model in the Bedrock service to generate a stream of chat responses. |
| `demo_claude_multimedia` | [src/examples/demo_claude_multimedia.rs](src/examples/demo_claude_multimedia.rs) | Demonstrates how to use the Claude model in the Bedrock service to generate a response based on text and an image. |
| `demo_ollama_embedding` | [src/examples/demo_ollama_embedding.rs](src/examples/demo_ollama_embedding.rs) | Demonstrates how to use the Ollama API to generate text embeddings. |
| `demo_mistral_stream` | [src/examples/demo_mistral_stream.rs](src/examples/demo_mistral_stream.rs) | Demonstrates how to use the Mistral model in the Bedrock service to generate a stream of responses. 

## Contributing

Contributions to Hiramu are welcome! If you encounter any issues, have suggestions for improvements, or want to add new features, please open an issue or submit a pull request on the [GitHub repository](https://github.com/raphaelmansuy/hiramu).

To contribute to the project, follow these steps:

1. Fork the repository and create a new branch for your changes.
2. Make your modifications and ensure that the code compiles successfully.
3. Write tests to cover your changes and ensure that all existing tests pass.
4. Update the documentation, including the README and API docs, if necessary.
5. Submit a pull request with a clear description of your changes and the problem they solve.

## License

Hiramu is licensed under the [MIT License](./LICENCE).

## Acknowledgements

Hiramu is built on top of the following libraries and APIs:

- [Ollama](https://ollama.com/)
- [AWS Bedrock API](https://aws.amazon.com/bedrock/)
- [reqwest](https://docs.rs/reqwest)
- [tokio](https://tokio.rs/)
- [serde](https://serde.rs/)

We would like to express our gratitude to the developers and maintainers of these projects for their excellent work and contributions to the Rust ecosystem.
