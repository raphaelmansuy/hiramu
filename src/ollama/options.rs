use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Options {
    pub num_keep: Option<u32>,
    pub seed: Option<u32>,
    pub num_predict: Option<u32>,
    pub top_k: Option<u32>,
    pub top_p: Option<f32>,
    pub tfs_z: Option<f32>,
    pub typical_p: Option<f32>,
    pub repeat_last_n: Option<u32>,
    pub temperature: Option<f32>,
    pub repeat_penalty: Option<f32>,
    pub presence_penalty: Option<f32>,
    pub frequency_penalty: Option<f32>,
    pub mirostat: Option<u32>,
    pub mirostat_tau: Option<f32>,
    pub mirostat_eta: Option<f32>,
    pub penalize_newline: Option<bool>,
    pub stop: Option<Vec<String>>,
    pub numa: Option<bool>,
    pub num_ctx: Option<u32>,
    pub num_batch: Option<u32>,
    pub num_gqa: Option<u32>,
    pub num_gpu: Option<u32>,
    pub main_gpu: Option<u32>,
    pub low_vram: Option<bool>,
    pub f16_kv: Option<bool>,
    pub vocab_only: Option<bool>,
    pub use_mmap: Option<bool>,
    pub use_mlock: Option<bool>,
    pub rope_frequency_base: Option<f32>,
    pub rope_frequency_scale: Option<f32>,
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
