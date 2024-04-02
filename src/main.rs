use hiramu::examples::demo_ollama::chat_response_loop;



#[tokio::main]
async fn main() {
    chat_response_loop().await;
}

