# Revision History

## [2024-04-05]

### 0.1.7
- Added support for the Mistral API in the Bedrock module.

## 0.1.15

Implement new generate_text method in OllamaClient and add options_builder support to GenerateRequestBuilder and ChatRequestBuilder

- Added a new method `generate_text` to the `OllamaClient` that collects the stream of responses from the `generate` method into a single string.
- Implemented the `merge_options` function to merge the options provided in the request builder and the options builder.
- Added the `options_builder` field to the `GenerateRequestBuilder` and `ChatRequestBuilder` to allow setting options using an `OptionsBuilder`.
- Added unit tests for the `merge_options` function.
- Added a new test case for the `generate_text` method to ensure the maximum number of predictions is respected.