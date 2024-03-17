use Hiramu::Hiramu;

#[tokio::test]
async fn test_generate() {
    let hiramu = Hiramu::new("http://localhost:11434"); 
    let response_stream = hiramu.generate("mistral", "Hello, world!").await;

    assert!(response_stream.is_ok());

    // Future tests could include mock server responses and more thorough checks
}