use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Options {
    // The number of tokens to keep from the beginning of the generation.
    pub num_keep: Option<u32>,

    // A seed for the random number generator to ensure reproducibility.
    pub seed: Option<u32>,

    // The number of tokens to predict during generation.
    pub num_predict: Option<u32>,

    // The number of highest probability vocabulary tokens to keep for top-k sampling.
    pub top_k: Option<u32>,

    // The cumulative probability for top-p sampling; also known as nucleus sampling.
    pub top_p: Option<f32>,

    // The z-score for truncation sampling.
    pub tfs_z: Option<f32>,

    // The value used to scale the logits before applying softmax during generation.
    pub typical_p: Option<f32>,

    // The number of tokens to consider for repetition penalties.
    pub repeat_last_n: Option<u32>,

    // The temperature to use for generation. Higher values mean more randomness.
    pub temperature: Option<f32>,

    // The penalty to apply to tokens that have been repeated.
    pub repeat_penalty: Option<f32>,

    // The penalty for using tokens that are present in the context.
    pub presence_penalty: Option<f32>,

    // The penalty for using tokens frequently in the generation.
    pub frequency_penalty: Option<f32>,

    // A parameter for controlling diversity via the mirostat algorithm.
    pub mirostat: Option<u32>,

    // The tau parameter for the mirostat algorithm, controlling diversity.
    pub mirostat_tau: Option<f32>,

    // The eta parameter for the mirostat algorithm, controlling diversity.
    pub mirostat_eta: Option<f32>,

    // Whether to penalize the generation of new lines.
    pub penalize_newline: Option<bool>,

    // A list of strings where the generation should stop.
    pub stop: Option<Vec<String>>,

    // Whether to use Non-Uniform Memory Access (NUMA) for memory allocation.
    pub numa: Option<bool>,

    // The number of context tokens to use for the generation.
    pub num_ctx: Option<u32>,

    // The number of batches to process in parallel.
    pub num_batch: Option<u32>,

    // The number of gradient accumulation steps for question-answering tasks.
    pub num_gqa: Option<u32>,

    // The number of GPUs to use for generation.
    pub num_gpu: Option<u32>,

    // The index of the main GPU to use for generation.
    pub main_gpu: Option<u32>,

    // Whether to use low VRAM mode.
    pub low_vram: Option<bool>,

    // Whether to use 16-bit floating-point for key/value pairs in attention layers.
    pub f16_kv: Option<bool>,

    // Whether to restrict generation to vocabulary tokens only.
    pub vocab_only: Option<bool>,

    // Whether to use memory-mapped files for model parameters.
    pub use_mmap: Option<bool>,

    // Whether to lock the model parameters in memory to prevent swapping.
    pub use_mlock: Option<bool>,

    // The base frequency for the ROPE sinusoidal positional encoding.
    pub rope_frequency_base: Option<f32>,

    // The scale for the ROPE sinusoidal positional encoding frequency.
    pub rope_frequency_scale: Option<f32>,

    // The number of threads to use for generation.
    pub num_thread: Option<u32>,
}

pub struct OptionsBuilder {
    inner: Options,
}

impl OptionsBuilder {
    pub fn new() -> Self {
        Self {
            inner: Options {
                num_keep: None,
                seed: None,
                num_predict: None,
                top_k: None,
                top_p: None,
                tfs_z: None,
                typical_p: None,
                repeat_last_n: None,
                temperature: None,
                repeat_penalty: None,
                presence_penalty: None,
                frequency_penalty: None,
                mirostat: None,
                mirostat_tau: None,
                mirostat_eta: None,
                penalize_newline: None,
                stop: None,
                numa: None,
                num_ctx: None,
                num_batch: None,
                num_gqa: None,
                num_gpu: None,
                main_gpu: None,
                low_vram: None,
                f16_kv: None,
                vocab_only: None,
                use_mmap: None,
                use_mlock: None,
                rope_frequency_base: None,
                rope_frequency_scale: None,
                num_thread: None,
            },
        }
    }

    pub fn num_keep(mut self, value: u32) -> Self {
        self.inner.num_keep = Some(value);
        self
    }

    pub fn seed(mut self, value: u32) -> Self {
        self.inner.seed = Some(value);
        self
    }

    pub fn num_predict(mut self, value: u32) -> Self {
        self.inner.num_predict = Some(value);
        self
    }

    pub fn top_k(mut self, value: u32) -> Self {
        self.inner.top_k = Some(value);
        self
    }

    pub fn top_p(mut self, value: f32) -> Self {
        self.inner.top_p = Some(value);
        self
    }

    pub fn tfs_z(mut self, value: f32) -> Self {
        self.inner.tfs_z = Some(value);
        self
    }

    pub fn typical_p(mut self, value: f32) -> Self {
        self.inner.typical_p = Some(value);
        self
    }

    pub fn repeat_last_n(mut self, value: u32) -> Self {
        self.inner.repeat_last_n = Some(value);
        self
    }

    pub fn temperature(mut self, value: f32) -> Self {
        self.inner.temperature = Some(value);
        self
    }

    pub fn repeat_penalty(mut self, value: f32) -> Self {
        self.inner.repeat_penalty = Some(value);
        self
    }

    pub fn presence_penalty(mut self, value: f32) -> Self {
        self.inner.presence_penalty = Some(value);
        self
    }

    pub fn frequency_penalty(mut self, value: f32) -> Self {
        self.inner.frequency_penalty = Some(value);
        self
    }

    pub fn mirostat(mut self, value: u32) -> Self {
        self.inner.mirostat = Some(value);
        self
    }

    pub fn mirostat_tau(mut self, value: f32) -> Self {
        self.inner.mirostat_tau = Some(value);
        self
    }

    pub fn mirostat_eta(mut self, value: f32) -> Self {
        self.inner.mirostat_eta = Some(value);
        self
    }

    pub fn penalize_newline(mut self, value: bool) -> Self {
        self.inner.penalize_newline = Some(value);
        self
    }

    pub fn stop(mut self, value: Vec<String>) -> Self {
        self.inner.stop = Some(value);
        self
    }

    pub fn numa(mut self, value: bool) -> Self {
        self.inner.numa = Some(value);
        self
    }

    pub fn num_ctx(mut self, value: u32) -> Self {
        self.inner.num_ctx = Some(value);
        self
    }

    pub fn num_batch(mut self, value: u32) -> Self {
        self.inner.num_batch = Some(value);
        self
    }

    pub fn num_gqa(mut self, value: u32) -> Self {
        self.inner.num_gqa = Some(value);
        self
    }

    pub fn num_gpu(mut self, value: u32) -> Self {
        self.inner.num_gpu = Some(value);
        self
    }

    pub fn main_gpu(mut self, value: u32) -> Self {
        self.inner.main_gpu = Some(value);
        self
    }

    pub fn low_vram(mut self, value: bool) -> Self {
        self.inner.low_vram = Some(value);
        self
    }

    pub fn f16_kv(mut self, value: bool) -> Self {
        self.inner.f16_kv = Some(value);
        self
    }

    pub fn vocab_only(mut self, value: bool) -> Self {
        self.inner.vocab_only = Some(value);
        self
    }

    pub fn use_mmap(mut self, value: bool) -> Self {
        self.inner.use_mmap = Some(value);
        self
    }

    pub fn use_mlock(mut self, value: bool) -> Self {
        self.inner.use_mlock = Some(value);
        self
    }

    pub fn rope_frequency_base(mut self, value: f32) -> Self {
        self.inner.rope_frequency_base = Some(value);
        self
    }

    pub fn rope_frequency_scale(mut self, value: f32) -> Self {
        self.inner.rope_frequency_scale = Some(value);
        self
    }

    pub fn num_thread(mut self, value: u32) -> Self {
        self.inner.num_thread = Some(value);
        self
    }

    pub fn build(self) -> Options {
        self.inner
    }
}
