use hiramu::Hiramu;
//use futures::StreamExt; 


#[tokio::main]
async fn main() {
    let hiramu = Hiramu::new("http://localhost:11434");

    let mut responses = Box::pin(hiramu.generate("mistral", "What is the capital of France ?").await.unwrap());

    while let Some(response) = responses.next().await {
        match response {
            Ok(generate_response) => println!("Received response: {:?}", generate_response),
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }


}