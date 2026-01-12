//! Patreon API client library.
//!
//! This crate provides two clients:
//! - `PatreonUserClient`: user-facing client for accessing user data after OAuth authorization
//! - `PatreonCreatorClient`: server-side client for creators to access campaigns, members, and posts
//!
//! # Examples
//!
//! ## User API (after OAuth authorization)
//! ```rust,ignore
//! use patreon::{PatreonUserClient, oauth::OAuthClient};
//!
//! // 1) Generate an authorization URL
//! let oauth_client = OAuthClient::new(
//!     "your_client_id",
//!     "your_client_secret",
//!     "https://your-app.com/callback",
//! );
//! let auth_url = oauth_client.authorization_url(&["identity", "identity.memberships"]);
//!
//! // 2) Exchange the authorization code for an access token (from your redirect handler)
//! let token = oauth_client.exchange_code("authorization_code").await?;
//!
//! // 3) Use the user client
//! let user_client = PatreonUserClient::new(&token.access_token);
//! let identity = user_client.identity().await?;
//! ```
//!
//! ## Server API (creator token)
//! ```rust,ignore
//! use patreon::PatreonCreatorClient;
//!
//! let creator_client = PatreonCreatorClient::new("creator_access_token");
//! let campaigns = creator_client.campaigns().await?;
//! let members = creator_client.campaign_members("campaign_id").await?;
//! ```

pub mod error;
pub mod models;
pub mod oauth;
pub mod user_client;
pub mod creator_client;
pub mod webhook;

pub use error::{Error, Result};
pub use models::*;
pub use oauth::OAuthClient;
pub use user_client::PatreonUserClient;
pub use creator_client::PatreonCreatorClient;
pub use webhook::WebhookValidator;

#[cfg(all(feature = "native-tls", feature = "rustls"))]
compile_error!("Enable only one TLS backend feature: `native-tls` or `rustls`.");

#[cfg(not(any(feature = "native-tls", feature = "rustls")))]
compile_error!("Enable one TLS backend feature: `native-tls` or `rustls`.");

/// Patreon API base URL.
pub const API_BASE_URL: &str = "https://www.patreon.com/api/oauth2/v2";

/// Patreon OAuth authorization endpoint URL.
pub const OAUTH_AUTHORIZE_URL: &str = "https://www.patreon.com/oauth2/authorize";

/// Patreon OAuth Token URL
pub const OAUTH_TOKEN_URL: &str = "https://www.patreon.com/api/oauth2/token";
