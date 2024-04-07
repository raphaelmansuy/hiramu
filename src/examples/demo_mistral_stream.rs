use futures::TryStreamExt;

use crate::bedrock::models::mistral::MistralClient;
use crate::bedrock::models::mistral::MistralOptions;
use crate::bedrock::models::mistral::MistralRequestBuilder;


pub async fn demo_mistra_with_stream(model_id: &str, prompt: &str) {

    let mistral_otions
     = MistralOptions::new()
        .profile_name("bedrock")
        .region("us-west-2");

    let client = MistralClient::new(mistral_otions).await;



    let request = MistralRequestBuilder::new(prompt.to_owned())
        .max_tokens(200)
        .temperature(0.5)
        .top_p(0.9)
        .top_k(100)
        .build();

    let response_stream = client
        .generate_with_stream(
            model_id.to_string(),
            &request
        )
        .await;
     
    let response_stream = match response_stream {
        Ok(response_stream) => response_stream,
        Err(e) => {
            println!("Error: {:?}", e);
            return;
        }
    };

    // consumme the stream and print the response
    response_stream
        .try_for_each(|chunk| async move {
            let json_display = serde_json::to_string_pretty(&chunk).unwrap();
            println!("{:?}", json_display);
            Ok(())
        })
        .await
        .unwrap();

}

// Test
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_demo_chat_mistral_with_stream() {
        let model_id = ModelInfo::from_model_name(ModelName::MistralMixtral8X7BInstruct0x);
        let prompt = "<s>[INST] What is the capital of France ?[/INST]";
        demo_mistra_with_stream(&model_id, prompt).await;
    }
}
