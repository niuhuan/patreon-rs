//! Webhook validation and parsing utilities.
//!
//! Patreon webhooks use an HMAC signature to validate request integrity.
//! The signature is provided in the `X-Patreon-Signature` header.

use crate::{Error, Result};
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// Webhook validator.
///
/// Validates signatures of incoming webhook requests.
///
/// # Example
///
/// ```rust,ignore
/// use patreon::WebhookValidator;
///
/// let validator = WebhookValidator::new("webhook_secret");
///
/// // Validate a webhook request
/// let is_valid = validator.validate(
///     &request_body,
///     &signature_header,
/// )?;
///
/// if is_valid {
///     // Parse the webhook event payload
///     let event = validator.parse_event(&request_body)?;
/// }
/// ```
#[derive(Debug, Clone)]
pub struct WebhookValidator {
    secret: String,
}

/// Webhook event.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WebhookEvent {
    /// Event data.
    pub data: serde_json::Value,
    /// Included related resources (JSON:API `included`).
    #[serde(default)]
    pub included: Option<Vec<serde_json::Value>>,
    /// Links (JSON:API `links`).
    #[serde(default)]
    pub links: Option<serde_json::Value>,
}

/// Webhook event type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WebhookEventType {
    /// Member created.
    MembersCreate,
    /// Member updated.
    MembersUpdate,
    /// Member deleted.
    MembersDelete,
    /// Pledge created.
    MembersPledgeCreate,
    /// Pledge updated.
    MembersPledgeUpdate,
    /// Pledge deleted.
    MembersPledgeDelete,
    /// Post published.
    PostsPublish,
    /// Post updated.
    PostsUpdate,
    /// Post deleted.
    PostsDelete,
    /// Unknown event type.
    Unknown(String),
}

impl WebhookEventType {
    /// Parses an event type from a string.
    pub fn from_str(s: &str) -> Self {
        match s {
            "members:create" => Self::MembersCreate,
            "members:update" => Self::MembersUpdate,
            "members:delete" => Self::MembersDelete,
            "members:pledge:create" => Self::MembersPledgeCreate,
            "members:pledge:update" => Self::MembersPledgeUpdate,
            "members:pledge:delete" => Self::MembersPledgeDelete,
            "posts:publish" => Self::PostsPublish,
            "posts:update" => Self::PostsUpdate,
            "posts:delete" => Self::PostsDelete,
            other => Self::Unknown(other.to_string()),
        }
    }

    /// Returns the event type as a string.
    pub fn as_str(&self) -> &str {
        match self {
            Self::MembersCreate => "members:create",
            Self::MembersUpdate => "members:update",
            Self::MembersDelete => "members:delete",
            Self::MembersPledgeCreate => "members:pledge:create",
            Self::MembersPledgeUpdate => "members:pledge:update",
            Self::MembersPledgeDelete => "members:pledge:delete",
            Self::PostsPublish => "posts:publish",
            Self::PostsUpdate => "posts:update",
            Self::PostsDelete => "posts:delete",
            Self::Unknown(s) => s,
        }
    }
}

impl WebhookValidator {
    /// Creates a new `WebhookValidator`.
    ///
    /// # Parameters
    /// - `secret`: webhook secret (generated when you create a webhook in the Patreon developer portal)
    pub fn new(secret: impl Into<String>) -> Self {
        Self {
            secret: secret.into(),
        }
    }

    /// Validates the webhook signature.
    ///
    /// # Parameters
    /// - `body`: raw request body bytes
    /// - `signature`: the value of the `X-Patreon-Signature` header
    ///
    /// # Returns
    /// `true` if the signature is valid, otherwise `false`.
    pub fn validate(&self, body: &[u8], signature: &str) -> bool {
        let expected = self.compute_signature(body);
        // Constant-time comparison to reduce timing side channels.
        constant_time_compare(&expected, signature)
    }

    /// Validates the webhook signature and returns an error on failure.
    ///
    /// # Parameters
    /// - `body`: raw request body bytes
    /// - `signature`: the value of the `X-Patreon-Signature` header
    ///
    /// # Errors
    /// Returns `Error::WebhookSignatureInvalid` if the signature is invalid.
    pub fn validate_or_error(&self, body: &[u8], signature: &str) -> Result<()> {
        if self.validate(body, signature) {
            Ok(())
        } else {
            Err(Error::WebhookSignatureInvalid)
        }
    }

    /// Computes the HMAC-SHA256 signature of the request body.
    fn compute_signature(&self, body: &[u8]) -> String {
        let mut mac = HmacSha256::new_from_slice(self.secret.as_bytes())
            .expect("HMAC can take key of any size");
        mac.update(body);
        let result = mac.finalize();
        hex::encode(result.into_bytes())
    }

    /// Parses a webhook event from a JSON string.
    ///
    /// # Parameters
    /// - `body`: request body as a string
    ///
    /// # Returns
    /// The parsed webhook event.
    pub fn parse_event(&self, body: &str) -> Result<WebhookEvent> {
        Ok(serde_json::from_str(body)?)
    }

    /// Parses a webhook event from raw bytes.
    pub fn parse_event_from_bytes(&self, body: &[u8]) -> Result<WebhookEvent> {
        Ok(serde_json::from_slice(body)?)
    }

    /// Validates and then parses a webhook event.
    ///
    /// Convenience method that validates the signature first and then parses the event.
    ///
    /// # Parameters
    /// - `body`: raw request body bytes
    /// - `signature`: the value of the `X-Patreon-Signature` header
    ///
    /// # Returns
    /// The parsed webhook event if the signature is valid.
    pub fn validate_and_parse(&self, body: &[u8], signature: &str) -> Result<WebhookEvent> {
        self.validate_or_error(body, signature)?;
        self.parse_event_from_bytes(body)
    }
}

/// Constant-time string comparison.
fn constant_time_compare(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut result = 0u8;
    for (x, y) in a.bytes().zip(b.bytes()) {
        result |= x ^ y;
    }
    result == 0
}

/// Webhook request header names.
pub mod headers {
    /// Event type header.
    pub const X_PATREON_EVENT: &str = "X-Patreon-Event";
    /// Signature header.
    pub const X_PATREON_SIGNATURE: &str = "X-Patreon-Signature";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_time_compare() {
        assert!(constant_time_compare("hello", "hello"));
        assert!(!constant_time_compare("hello", "world"));
        assert!(!constant_time_compare("hello", "hell"));
    }

    #[test]
    fn test_webhook_event_type() {
        assert_eq!(
            WebhookEventType::from_str("members:create"),
            WebhookEventType::MembersCreate
        );
        assert_eq!(
            WebhookEventType::from_str("unknown:event"),
            WebhookEventType::Unknown("unknown:event".to_string())
        );
    }

    #[test]
    fn test_compute_signature() {
        let validator = WebhookValidator::new("test_secret");
        let body = b"test body";
        let signature = validator.compute_signature(body);
        assert!(!signature.is_empty());
        // Signature should be a 64-char hex string (SHA-256).
        assert_eq!(signature.len(), 64);
    }

    #[test]
    fn test_validate() {
        let validator = WebhookValidator::new("test_secret");
        let body = b"test body";
        let signature = validator.compute_signature(body);
        assert!(validator.validate(body, &signature));
        assert!(!validator.validate(body, "invalid_signature"));
    }
}
