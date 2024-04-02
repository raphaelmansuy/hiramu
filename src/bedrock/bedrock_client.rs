use crate::bedrock::bedrock_error::BedrockError;
use aws_config::Region;
use aws_sdk_bedrock::config::BehaviorVersion;
use aws_sdk_bedrockruntime::{Client, Error};
use futures::stream::Stream;
use serde_json::Value;
use std::borrow::Cow;
use tokio_stream::wrappers::UnboundedReceiverStream;

#[derive(Debug, Clone)]
pub struct BedrockClientOptions {
    profile_name: Option<String>,
    region: Option<String>,
    endpoint_url: Option<String>,
    behavior_version: Option<BehaviorVersion>,
}

impl BedrockClientOptions {
    pub fn new() -> Self {
        Self {
            profile_name: None,
            region: Some("us-west-2".to_string()),
            endpoint_url: None,
            behavior_version: Some(BehaviorVersion::v2023_11_09()),
        }
    }

    pub fn profile_name<S: Into<String>>(mut self, profile_name: S) -> Self {
        self.profile_name = Some(profile_name.into());
        self
    }

    pub fn region<S: Into<String>>(mut self, region: S) -> Self {
        self.region = Some(region.into());
        self
    }

    pub fn endpoint_url<S: Into<String>>(mut self, endpoint_url: S) -> Self {
        self.endpoint_url = Some(endpoint_url.into());
        self
    }

    pub fn behavior_version(mut self, behavior_version: BehaviorVersion) -> Self {
        self.behavior_version = Some(behavior_version);
        self
    }
}

pub struct BedrockClient {
    client: Client,
}

impl BedrockClient {
    /// Constructs a new `BedrockClient`.
    pub async fn new(options: BedrockClientOptions) -> Self {
        let client = Self::create_client(options).await;
        Self { client }
    }

    async fn create_client(options: BedrockClientOptions) -> Client {
        let mut config_loader = aws_config::ConfigLoader::default();

        if let Some(profile_name) = options.profile_name {
            config_loader = config_loader.profile_name(profile_name);
        }

        if let Some(region) = options.region {
            config_loader = config_loader.region(Region::new(region));
        }

        if let Some(endpoint_url) = options.endpoint_url {
            config_loader = config_loader.endpoint_url(endpoint_url);
        }

        if let Some(behavior_version) = options.behavior_version {
            config_loader = config_loader.behavior_version(behavior_version);
        }

        let config = config_loader.load().await;

        Client::new(&config)
    }

    pub async fn generate_raw_stream(
        &self,
        model_id: String,
        payload: Value,
    ) -> Result<impl Stream<Item = Result<Value, BedrockError>>, BedrockError> {
        let payload_bytes = serde_json::to_vec(&payload);

        let payload_bytes = match payload_bytes {
            Ok(bytes) => bytes,
            Err(err) => return Err(BedrockError::from(err)),
        };

        let payload_blob = aws_smithy_types::Blob::new(payload_bytes);

        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();

        let client = self.client.clone();

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
                                sender
                                    .send(Err(BedrockError::ApiError(err.to_string())))
                                    .unwrap();
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
                    sender
                        .send(Err(BedrockError::ApiError(err.to_string())))
                        .unwrap();
                }
            }
        });

        Ok(UnboundedReceiverStream::new(receiver))
    }

    pub async fn generate_raw(&self, model_id: String, payload: Value) -> Result<Value, Error> {
        let payload_bytes = serde_json::to_vec(&payload).unwrap();
        let payload_blob = aws_smithy_types::Blob::new(payload_bytes);

        let client = self.client.clone();

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
