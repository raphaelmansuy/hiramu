use aws_config::Region;
use aws_sdk_bedrock::config::BehaviorVersion;
use aws_sdk_bedrockruntime::{Client, Error};
use futures::stream::Stream;
use serde_json::Value;
use std::borrow::Cow;
use tokio_stream::wrappers::UnboundedReceiverStream;

pub struct BedrockClient {}

impl BedrockClient {
    /// Constructs a new `BedrockClient`.
    pub fn new() -> Self {
        Self {}
    }

    async fn create_client(profile_name: Option<&str>, region: Option<&str>) -> Client {
        let profile = profile_name.unwrap_or("default");
        let region_name = region.unwrap_or("us-west-2");
        let region = Region::new(region_name.to_string());

        // Create a new config loader
        let config = aws_config::defaults(BehaviorVersion::v2023_11_09())
            .profile_name(profile)
            .region(region)
            .load()
            .await;

        // Create a shared config
        Client::new(&config)
    }

    pub async fn generate_raw_stream(
        &self,
        model_id: String,
        payload: Value,
        profile_name: Option<String>,
        region: Option<String>,
    ) -> impl Stream<Item = Result<Value, Error>> {
        let client = Self::create_client(profile_name.as_deref(), region.as_deref()).await;

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
                                    let data: Cow<'_, str> =
                                        String::from_utf8_lossy(&blob.as_ref());
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
                            Ok(Some(_)) => {}
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
        &self,
        model_id: String,
        payload: Value,
        profile_name: Option<String>,
        region: Option<String>,
    ) -> Result<Value, Error> {
        let client = Self::create_client(profile_name.as_deref(), region.as_deref()).await;

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

        let response: serde_json::Value = serde_json::from_slice(resp.body().as_ref()).unwrap();
        Ok(response)
    }
}
