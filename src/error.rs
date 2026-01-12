//! Error types.

use thiserror::Error;

/// Patreon API error type.
#[derive(Error, Debug)]
pub enum Error {
    /// HTTP request error.
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    /// JSON serialization/deserialization error.
    #[error("JSON parse failed: {0}")]
    Json(#[from] serde_json::Error),

    /// API error response.
    #[error("API error: {status} - {message}")]
    Api {
        /// HTTP status code.
        status: u16,
        /// Error message.
        message: String,
    },

    /// OAuth error.
    #[error("OAuth error: {error} - {description}")]
    OAuth {
        /// Error type.
        error: String,
        /// Error description.
        description: String,
    },

    /// Webhook signature validation failed.
    #[error("Webhook signature validation failed")]
    WebhookSignatureInvalid,

    /// Missing required header.
    #[error("Missing required header: {0}")]
    MissingHeader(String),

    /// Invalid response format.
    #[error("Invalid response format: {0}")]
    InvalidResponse(String),
}

/// Result type alias.
pub type Result<T> = std::result::Result<T, Error>;
