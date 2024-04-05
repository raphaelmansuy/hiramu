use futures::TryStreamExt;
use std::io;
use std::io::Write;

use crate::bedrock::bedrock_client::{BedrockClient, BedrockClientOptions};


pub async fn demo_bedrock_mistral_raw_stream(model_id: &str, prompt: &str) {
    let profile_name = "bedrock";
    let region = "us-west-2";


    let payload = serde_json::json!({
        "prompt": prompt,
        "max_tokens" : 200,
        "stop" : ["[INST]"],    
        "temperature": 0.5,
        "top_p": 0.9,
        "top_k": 100,
    });

    let options = BedrockClientOptions::new()
        .profile_name(profile_name)
        .region(region);
    

    let client = BedrockClient::new(options).await;

    let stream = client
        .generate_raw_stream(
            model_id.to_string(),
            payload,
        )
        .await;

    let stream = match stream {
        Ok(stream) => stream,
        Err(err) => {
            println!("Error: {:?}", err);
            return;
        }
    };

    // consumme the stream and print the response
    stream
        .try_for_each(|chunk| async move {
            println!("{:?}", chunk);
            // Flush the output to ensure the prompt is displayed.
            io::stdout().flush().unwrap();
            Ok(())
        })
        .await
        .unwrap();
}

// Write a test

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bedrock::model_info::{ModelInfo, ModelName};

    #[tokio::test]
    async fn test_demo_bedrock_mistral_raw_stream_8x7() {
        let model_id = ModelInfo::from_model_name(ModelName::MistralMixtral8X7BInstruct0x);
        let prompt = "<s>[INST] What is your favourite condiment? [/INST]";
        demo_bedrock_mistral_raw_stream(&model_id,&prompt).await;
    }

    #[tokio::test]
    async fn test_demo_bedrock_mistral_raw_stream_7b() {
        let model_id = ModelInfo::from_model_name(ModelName::MistralMistral7BInstruct0x);
        let prompt = "<s>[INST] What is your favourite condiment? [/INST]";
        demo_bedrock_mistral_raw_stream(&model_id,&prompt).await;
    }

    #[tokio::test]
    async fn test_demo_bedrock_mistral_raw_stream_large() {
        let model_id = ModelInfo::from_model_name(ModelName::MistralLarge);
        let prompt = "<s>[INST] What is your favourite condiment? [/INST]";
        demo_bedrock_mistral_raw_stream(&model_id,&prompt).await;
    }


}
