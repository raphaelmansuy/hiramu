use serde::{Deserialize, Serialize};

/// Represents the response from the Claude AI assistant.
/// The Anthropic Claude model returns the following fields for a Text Completion inference call.
#[derive(Deserialize, Serialize, Debug)]
pub struct ClaudeResponse {
    /// The resulting completion up to and excluding the stop sequences.
    pub completion: String,
    /// The reason why the model stopped generating the response.
    pub stop_reason: String,
    /// The model reached a stop sequence — either provided by you with the stop_sequences inference parameter, or a stop sequence built into the model.
    pub stop: String,
}

impl ClaudeResponse {
    /// Creates a new `CauseResponse` instance with the given parameters.
    ///
    /// # Arguments
    ///
    /// * `completion` - The resulting completion up to and excluding the stop sequences.
    /// * `stop_reason` - The reason why the model stopped generating the response.
    /// * `stop` - The stop sequence that signalled the model to stop generating text.
    pub fn new(completion: &str, stop_reason: &str, stop: &str) -> Self {
        ClaudeResponse {
            completion: completion.to_string(),
            stop_reason: stop_reason.to_string(),
            stop: stop.to_string(),
        }
    }

    /// Checks if the response is complete, i.e., the model stopped due to a stop sequence.
    ///
    /// # Returns
    ///
    /// * `true` if the response is complete, `false` otherwise.
    pub fn is_complete(&self) -> bool {
        self.stop_reason == "stop_sequence"
    }

    /// Checks if the response is truncated, i.e., the model stopped due to reaching the maximum number of tokens.
    ///
    /// # Returns
    ///
    /// * `true` if the response is truncated, `false` otherwise.
    pub fn is_truncated(&self) -> bool {
        self.stop_reason == "max_tokens"
    }
}