use aws_config::meta::region::RegionProviderChain;
use aws_sdk_bedrockruntime::{Client, Error};
use futures::stream::{self, Stream, StreamExt};
use serde_json::Value;
use std::env;
use std::{borrow::Cow, io::Write};
use futures_util::future;
use tokio_stream::{ wrappers::UnboundedReceiverStream};

pub struct BedrockClient {}

impl BedrockClient {
    /// Constructs a new `BedrockClient`.
    pub fn new() -> Self {
        Self {}
    }

    pub async fn generate_raw_stream(
        model_id: String,
        profile_name: String,
        region: String,
        payload: Value,
    ) -> impl Stream<Item = Result<Value, Error>> {
        // Set AWS_PROFILE environment variable
        env::set_var("AWS_PROFILE", profile_name);
    
        // Set AWS_REGION environment variable
        env::set_var("AWS_REGION", region);
    
        let region_provider = RegionProviderChain::default_provider().or_else("us-west-2");
        let shared_config = aws_config::from_env().region(region_provider).load().await;
    
        let client = Client::new(&shared_config);
        let payload_bytes = serde_json::to_vec(&payload).unwrap();
        let payload_blob = aws_smithy_types::Blob::new(payload_bytes);
    
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();
    
        tokio::spawn(async move {
            let resp = client
                .invoke_model_with_response_stream()
                .model_id(&model_id)
                .content_type("application/json")
                .body(payload_blob)
                .send()
                .await;
    
            match resp {
                Ok(output) => {
                    let mut response_stream = output.body;

                    loop {
                        match response_stream.recv().await {
                            Ok(Some(aws_sdk_bedrockruntime::types::ResponseStream::Chunk(
                                payload_part,
                            ))) => {
                                if let Some(blob) = &payload_part.bytes {
                                    let data: Cow<'_, str> = String::from_utf8_lossy(&blob.as_ref());
                                    let value: Value = serde_json::from_str(&data).unwrap();
                                    sender.send(Ok(value)).unwrap();
                                }
                            }
                            Err(err) => {
                                sender.send(Err(Error::from(err))).unwrap();
                                break;
                            }
                            Ok(None) => {
                                break;
                            }
                            Ok(Some(_)) => {
                            }
                        }
                    }
                }
                Err(err) => {
                    sender.send(Err(Error::from(err))).unwrap();
                }
            }
        });
    
        UnboundedReceiverStream::new(receiver)
    }


    pub async fn generate_raw(
        model_id: String,
        profile_name: String,
        region: String,
        payload: Value,
    ) -> Result<Value, Error> {
        // Set AWS_PROFILE environment variable
        env::set_var("AWS_PROFILE", profile_name);

        // Set AWS_REGION environment variable
        env::set_var("AWS_REGION", region);

        let region_provider = RegionProviderChain::default_provider().or_else("us-west-2");
        let shared_config = aws_config::from_env().region(region_provider).load().await;

        // Create a new Bedrock Runtime client
        let client = Client::new(&shared_config);

        let payload_bytes = serde_json::to_vec(&payload).unwrap();
        let payload_blob = aws_smithy_types::Blob::new(payload_bytes);

        // Invoke the model with the payload
        let resp = client
            .invoke_model()
            .model_id(model_id)
            .content_type("application/json")
            .body(payload_blob)
            .send()
            .await?;

        // Print the model's response
        let response: serde_json::Value = serde_json::from_slice(resp.body().as_ref()).unwrap();
        Ok(response)
    }

    pub async fn generate() -> Result<(), Error> {
        // set environment variables AWS_PROFILE and AWS_REGION

        // Set AWS_PROFILE environment variable
        env::set_var("AWS_PROFILE", "bedrock");

        // Set AWS_REGION environment variable
        env::set_var("AWS_REGION", "us-west-2");

        let region_provider = RegionProviderChain::default_provider().or_else("us-west-2");
        let shared_config = aws_config::from_env().region(region_provider).load().await;

        // Create a new Bedrock Runtime client
        let client = Client::new(&shared_config);

        // Define the model ID and input prompt
        let model_id = "anthropic.claude-3-haiku-20240307-v1:0";
        let prompt = "Hi. In a short paragraph, explain what you can do.";

        // Prepare the payload for the model
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

        let payload_bytes = serde_json::to_vec(&payload).unwrap();

        let payload_blob = aws_smithy_types::Blob::new(payload_bytes);

        let output_stream = client
            .invoke_model_with_response_stream()
            .model_id(model_id)
            .content_type("application/json")
            .body(payload_blob.clone())
            .send()
            .await;

        match output_stream {
            Ok(output) => {
                let mut response_stream = output.body;
                loop {
                    match response_stream.recv().await {
                        Ok(Some(aws_sdk_bedrockruntime::types::ResponseStream::Chunk(
                            payload_part,
                        ))) => {
                            if let Some(blob) = &payload_part.bytes {
                                let data: Cow<'_, str> = String::from_utf8_lossy(&blob.as_ref());
                                let value: Value = serde_json::from_str(&data).unwrap();
                                if value["type"] == "content_block_delta" {
                                    if let Some(delta) = value["delta"].as_object() {
                                        if let Some(text) = delta["text"].as_str() {
                                            print!("{}", text);
                                            // flush the output buffer
                                            std::io::stdout().flush().unwrap();
                                        }
                                    }
                                }
                            }
                        }
                        Err(err) => {
                            println!("Stream Error");
                        }
                        Ok(None) => {
                            println!("Stream End");
                            break;
                        }
                        Ok(Some(_)) => {
                            println!("other case");
                        }
                    }
                }
            }
            Err(err) => {
                eprintln!("Bedrock Error: {:?}", err);
            }
        }

        Ok(())
    }
}
