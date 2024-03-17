use hiramu::Hiramu;

#[tokio::main]
async fn main() {
    let hiramu = Hiramu::new("http://localhost:11434");
    let responses = match hiramu.generate("mistral", "What is the capital of France ?").await {
        Ok(responses) => responses,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    for response in responses {
        println!("{}", response.response);
    }
}