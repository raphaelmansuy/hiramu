use futures::TryStreamExt;
use std::io::{self, Write};

use crate::ollama::{ChatRequestBuilder, Message, OllamaClient, OllamaError, OptionsBuilder};

pub async fn demo_chat_with_ollama_with_stream() -> Result<(), OllamaError> {
    let client = OllamaClient::new("http://localhost:11434".to_string());

    let messages = vec![Message::new(
        "user".to_string(),
        "What is the capital of France?  "
            .to_string(),
    )];

    let options = OptionsBuilder::new()
        .num_predict(100) // Limit the number of predicted tokens
        .temperature(0.4);

    let request = ChatRequestBuilder::new("mistral".to_string())
        .messages(messages.to_owned())
        .options_from_builder(options)
        .build();

    let response_stream = client.chat(request).await?;

    let result = response_stream
        .try_for_each(|chunk| async {
            let message = chunk.message;
            print!("{}", message.content);
            // Flush the output to ensure the prompt is displayed.
            io::stdout().flush().unwrap();
            Ok(())
        })
        .await;

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_demo_chat_with_ollama_with_stream() {
        let _ = demo_chat_with_ollama_with_stream().await;
    }
}
