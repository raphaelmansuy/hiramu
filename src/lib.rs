//! # Hiramu
//!
#![doc = include_str!("../README.md")]

pub mod ollama;
pub mod bedrock;
pub mod error;
pub mod util;
pub mod examples;

pub use error::HiramuError;
pub use util::fetch_and_base64_encode_image;