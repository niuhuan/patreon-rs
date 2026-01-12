//! OAuth authentication module.
//!
//! Implements the Patreon OAuth 2.0 authorization flow.

use crate::{Error, Result, OAUTH_AUTHORIZE_URL, OAUTH_TOKEN_URL};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

/// OAuth client.
///
/// Used to implement the OAuth authorization flow.
#[derive(Debug, Clone)]
pub struct OAuthClient {
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    http_client: reqwest::Client,
}

/// OAuth token response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenResponse {
    /// Access token.
    pub access_token: String,
    /// Refresh token.
    pub refresh_token: String,
    /// Expires in seconds.
    pub expires_in: i64,
    /// Token type.
    pub token_type: String,
    /// Granted scopes (space-separated).
    pub scope: String,
    /// Token version.
    #[serde(default)]
    pub version: Option<String>,
}

/// OAuth token with an absolute expiration time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthToken {
    /// Access token.
    pub access_token: String,
    /// Refresh token.
    pub refresh_token: String,
    /// Expiration timestamp.
    pub expires_at: DateTime<Utc>,
    /// Token type.
    pub token_type: String,
    /// Granted scopes (space-separated).
    pub scope: String,
}

impl OAuthToken {
    /// Creates an `OAuthToken` from a `TokenResponse`.
    pub fn from_response(response: TokenResponse) -> Self {
        let expires_at = Utc::now() + Duration::seconds(response.expires_in);
        Self {
            access_token: response.access_token,
            refresh_token: response.refresh_token,
            expires_at,
            token_type: response.token_type,
            scope: response.scope,
        }
    }

    /// Returns `true` if the token is already expired.
    pub fn is_expired(&self) -> bool {
        Utc::now() >= self.expires_at
    }

    /// Returns `true` if the token will expire soon (within 5 minutes).
    pub fn is_expiring_soon(&self) -> bool {
        self.is_expiring_within(Duration::minutes(5))
    }

    /// Returns `true` if the token will expire within the given duration.
    pub fn is_expiring_within(&self, duration: Duration) -> bool {
        Utc::now() + duration >= self.expires_at
    }
}

/// OAuth error response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthErrorResponse {
    /// Error type.
    pub error: String,
    /// Optional error description.
    #[serde(default)]
    pub error_description: Option<String>,
}

/// OAuth scopes.
pub mod scopes {
    /// User identity.
    pub const IDENTITY: &str = "identity";
    /// User email.
    pub const IDENTITY_EMAIL: &str = "identity[email]";
    /// User memberships.
    pub const IDENTITY_MEMBERSHIPS: &str = "identity.memberships";
    /// Campaigns.
    pub const CAMPAIGNS: &str = "campaigns";
    /// Campaign members.
    pub const CAMPAIGNS_MEMBERS: &str = "campaigns.members";
    /// Campaign members email.
    pub const CAMPAIGNS_MEMBERS_EMAIL: &str = "campaigns.members[email]";
    /// Campaign members address.
    pub const CAMPAIGNS_MEMBERS_ADDRESS: &str = "campaigns.members.address";
    /// Campaign posts.
    pub const CAMPAIGNS_POSTS: &str = "campaigns.posts";
    /// Campaign webhooks.
    pub const CAMPAIGNS_WEBHOOK: &str = "w:campaigns.webhook";
}

impl OAuthClient {
    /// Creates a new OAuth client.
    ///
    /// # Parameters
    /// - `client_id`: OAuth client ID
    /// - `client_secret`: OAuth client secret
    /// - `redirect_uri`: OAuth redirect/callback URL
    pub fn new(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        redirect_uri: impl Into<String>,
    ) -> Self {
        Self {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            redirect_uri: redirect_uri.into(),
            http_client: reqwest::Client::new(),
        }
    }

    /// Builds an authorization URL.
    ///
    /// # Parameters
    /// - `scopes`: requested scopes
    ///
    /// # Returns
    /// The URL the user should visit to authorize your application.
    pub fn authorization_url(&self, scopes: &[&str]) -> String {
        let scope = scopes.join(" ");
        format!(
            "{}?response_type=code&client_id={}&redirect_uri={}&scope={}",
            OAUTH_AUTHORIZE_URL,
            urlencoding::encode(&self.client_id),
            urlencoding::encode(&self.redirect_uri),
            urlencoding::encode(&scope)
        )
    }

    /// Builds an authorization URL with a `state` parameter (recommended).
    ///
    /// # Parameters
    /// - `scopes`: requested scopes
    /// - `state`: anti-CSRF state value
    ///
    /// # Returns
    /// The URL the user should visit to authorize your application.
    pub fn authorization_url_with_state(&self, scopes: &[&str], state: &str) -> String {
        let scope = scopes.join(" ");
        format!(
            "{}?response_type=code&client_id={}&redirect_uri={}&scope={}&state={}",
            OAUTH_AUTHORIZE_URL,
            urlencoding::encode(&self.client_id),
            urlencoding::encode(&self.redirect_uri),
            urlencoding::encode(&scope),
            urlencoding::encode(state)
        )
    }

    /// Exchanges an authorization code for an access token.
    ///
    /// # Parameters
    /// - `code`: authorization code from your redirect/callback handler
    ///
    /// # Returns
    /// An `OAuthToken`.
    pub async fn exchange_code(&self, code: &str) -> Result<OAuthToken> {
        let params = [
            ("code", code),
            ("grant_type", "authorization_code"),
            ("client_id", &self.client_id),
            ("client_secret", &self.client_secret),
            ("redirect_uri", &self.redirect_uri),
        ];

        let response = self
            .http_client
            .post(OAUTH_TOKEN_URL)
            .form(&params)
            .send()
            .await?;

        if response.status().is_success() {
            let token_response: TokenResponse = response.json().await?;
            Ok(OAuthToken::from_response(token_response))
        } else {
            let error: OAuthErrorResponse = response.json().await?;
            Err(Error::OAuth {
                error: error.error,
                description: error.error_description.unwrap_or_default(),
            })
        }
    }

    /// Exchanges a refresh token for a new access token.
    ///
    /// # Parameters
    /// - `refresh_token`: refresh token
    ///
    /// # Returns
    /// A new `OAuthToken`.
    pub async fn refresh_token(&self, refresh_token: &str) -> Result<OAuthToken> {
        let params = [
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
            ("client_id", &self.client_id),
            ("client_secret", &self.client_secret),
        ];

        let response = self
            .http_client
            .post(OAUTH_TOKEN_URL)
            .form(&params)
            .send()
            .await?;

        if response.status().is_success() {
            let token_response: TokenResponse = response.json().await?;
            Ok(OAuthToken::from_response(token_response))
        } else {
            let error: OAuthErrorResponse = response.json().await?;
            Err(Error::OAuth {
                error: error.error,
                description: error.error_description.unwrap_or_default(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authorization_url() {
        let client = OAuthClient::new("test_client_id", "test_secret", "https://example.com/callback");
        let url = client.authorization_url(&[scopes::IDENTITY, scopes::IDENTITY_MEMBERSHIPS]);
        
        assert!(url.contains("client_id=test_client_id"));
        assert!(url.contains("redirect_uri=https%3A%2F%2Fexample.com%2Fcallback"));
        assert!(url.contains("scope=identity%20identity.memberships"));
    }

    #[test]
    fn test_authorization_url_with_state() {
        let client = OAuthClient::new("test_client_id", "test_secret", "https://example.com/callback");
        let url = client.authorization_url_with_state(&[scopes::IDENTITY], "random_state");
        
        assert!(url.contains("state=random_state"));
    }
}
