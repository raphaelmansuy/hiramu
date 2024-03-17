pub mod client;
pub mod models;
pub mod chat;
pub mod error;

pub use client::HiramuClient;
pub use models::GenerateResponse;
pub use models::GenerateRequest;
pub use chat::Chat;
pub use error::HiramuError;

#[cfg(test)]
mod tests;