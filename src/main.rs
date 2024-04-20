use hiramu::examples::demo_ollama::chat_response_loop;

#[tokio::main]
async fn main() {
    // A simple example that demonstrates how to use the Ollama API to generate responses to chat messages.
    chat_response_loop(u32::max_value(),None).await;
}
