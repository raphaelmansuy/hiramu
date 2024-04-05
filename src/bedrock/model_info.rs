#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum ModelName {
    AmazonTitanTextG1Express1x,
    AmazonTitanTextG1Lite1x,
    AmazonTitanEmbeddingsG1Text1x,
    AmazonTitanMultimodalEmbeddingsG1x,
    AmazonTitanImageGeneratorG1x,
    AnthropicClaude2x,
    AnthropicClaudeSonnet1x,
    AnthropicClaudeHaiku1x,
    AnthropicClaudeInstantx,
    AI21JurassicMid1x,
    AI21JurassicUltra1x,
    CohereCmdTxt14x,
    CohereCmdLightTxt15x,
    CohereEmbedEnglish3x,
    CohereEmbedMultilingual3x,
    MetaLlama2Chat13B1x,
    MetaLlama2Chat70B1x,
    MistralMistral7BInstruct0x,
    MistralMixtral8X7BInstruct0x,
    MistralLarge,
    StabilityStableDiffusionXL0x,
    StabilityStableDiffusionXL1x,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ModelInfo {
    pub name: ModelName,
    pub text: &'static str,
}

impl ModelInfo {
    pub const MODELS: &'static [ModelInfo] = &[
        ModelInfo {
            name: ModelName::AmazonTitanTextG1Express1x,
            text: "amazon.titan-text-express-v1",
        },
        ModelInfo {
            name: ModelName::AmazonTitanTextG1Lite1x,
            text: "amazon.titan-text-lite-v1",
        },
        ModelInfo {
            name: ModelName::AmazonTitanEmbeddingsG1Text1x,
            text: "amazon.titan-embed-text-v1",
        },
        ModelInfo {
            name: ModelName::AmazonTitanMultimodalEmbeddingsG1x,
            text: "amazon.titan-embed-image-v1",
        },
        ModelInfo {
            name: ModelName::AmazonTitanImageGeneratorG1x,
            text: "amazon.titan-image-generator-v1",
        },
        ModelInfo {
            name: ModelName::AnthropicClaude2x,
            text: "anthropic.claude-v2",
        },
        ModelInfo {
            name: ModelName::AnthropicClaudeSonnet1x,
            text: "anthropic.claude-3-sonnet-20240229-v1:0",
        },
        ModelInfo {
            name: ModelName::AnthropicClaudeHaiku1x,
            text: "anthropic.claude-3-haiku-20240307-v1:0",
        },
        ModelInfo {
            name: ModelName::AnthropicClaudeInstantx,
            text: "anthropic.claude-instant-v1",
        },
        ModelInfo {
            name: ModelName::AI21JurassicMid1x,
            text: "ai21.j2-mid-v1",
        },
        ModelInfo {
            name: ModelName::AI21JurassicUltra1x,
            text: "ai21.j2-ultra-v1",
        },
        ModelInfo {
            name: ModelName::CohereCmdTxt14x,
            text: "cohere.command-text-v14",
        },
        ModelInfo {
            name: ModelName::CohereCmdLightTxt15x,
            text: "cohere.command-light-text-v14",
        },
        ModelInfo {
            name: ModelName::CohereEmbedEnglish3x,
            text: "cohere.embed-english-v3",
        },
        ModelInfo {
            name: ModelName::CohereEmbedMultilingual3x,
            text: "cohere.embed-multilingual-v3",
        },
        ModelInfo {
            name: ModelName::MetaLlama2Chat13B1x,
            text: "meta.llama2-13b-chat-v1",
        },
        ModelInfo {
            name: ModelName::MetaLlama2Chat70B1x,
            text: "meta.llama2-70b-chat-v1",
        },
        ModelInfo {
            name: ModelName::MistralMistral7BInstruct0x,
            text: "mistral.mistral-7b-instruct-v0:2",
        },
        ModelInfo {
            name: ModelName::MistralMixtral8X7BInstruct0x,
            text: "mistral.mixtral-8x7b-instruct-v0:1",
        },
        ModelInfo {
            name: ModelName::MistralLarge,
            text: "mistral.mistral-large-2402-v1:0",
        },
        ModelInfo {
            name: ModelName::StabilityStableDiffusionXL0x,
            text: "stability.stable-diffusion-xl-v0",
        },
        ModelInfo {
            name: ModelName::StabilityStableDiffusionXL1x,
            text: "stability.stable-diffusion-xl-v1",
        },
    ];

    pub fn from_model_name(name: ModelName) -> String  {
        ModelInfo::MODELS.iter().find(|model| model.name == name).unwrap().text.to_string()
    }

}