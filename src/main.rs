use hiramu::HiramuClient;
use hiramu::Chat;
//use hiramu::GenerateResponse;
use tokio;

#[tokio::main]
async fn main() {
    //let client = HiramuClient::new("http://localhost:11434".to_string());
    let client = HiramuClient::new("http://localhost:11434".to_string());
    let mut chat = Chat::new(client, "The assistant will act like a pirate".to_string());

    loop {
        let input = prompt_input("\n> ").unwrap();
        let response = chat.add_message(input).await.unwrap();
        println!();
    }
}

fn prompt_input(prompt: &str) -> Result<String, std::io::Error> {
    use std::io::{self, Write};
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}
