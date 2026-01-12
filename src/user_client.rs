//! User API client.
//!
//! Used to access user-related data after a user completes the OAuth flow.
//! These APIs require a user `access_token`.
//!
//! ## Typical use cases
//! - Fetch the user's Patreon identity
//! - Check whether a user is a patron of a specific creator
//! - List memberships for the authorized user
//!
//! ## Available API
//! - `/api/oauth2/v2/identity` - Fetch the currently authorized user

use crate::models::*;
use crate::{Error, Result, API_BASE_URL};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};

/// Patreon user API client.
///
/// Accesses endpoints available to an OAuth-authorized user, including identity and memberships.
///
/// # Example
///
/// ```rust,ignore
/// use patreon::PatreonUserClient;
///
/// let client = PatreonUserClient::new("user_access_token");
///
/// // Fetch basic identity info
/// let identity = client.identity().await?;
///
/// // Fetch identity with memberships
/// let identity_with_memberships = client
///     .identity_with_memberships()
///     .await?;
/// ```
#[derive(Debug, Clone)]
pub struct PatreonUserClient {
    access_token: String,
    http_client: reqwest::Client,
    base_url: String,
}

impl PatreonUserClient {
    /// Creates a new user client.
    ///
    /// # Parameters
    /// - `access_token`: the user's OAuth access token
    pub fn new(access_token: impl Into<String>) -> Self {
        Self {
            access_token: access_token.into(),
            http_client: reqwest::Client::new(),
            base_url: API_BASE_URL.to_string(),
        }
    }

    /// Uses a custom `reqwest::Client`.
    pub fn with_http_client(mut self, client: reqwest::Client) -> Self {
        self.http_client = client;
        self
    }

    /// Uses a custom base URL (useful for tests).
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// Builds authorization headers.
    fn auth_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.access_token))
                .expect("Invalid token"),
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers
    }

    /// Sends a GET request.
    async fn get<T: serde::de::DeserializeOwned>(&self, endpoint: &str) -> Result<T> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self
            .http_client
            .get(&url)
            .headers(self.auth_headers())
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            let status = response.status().as_u16();
            let text = response.text().await.unwrap_or_default();
            Err(Error::Api {
                status,
                message: text,
            })
        }
    }

    // ==================== Identity API ====================

    /// Fetches the current authorized user's identity.
    ///
    /// Returns basic user data (no included related resources).
    ///
    /// # Required scopes
    /// - `identity`
    pub async fn identity(&self) -> Result<SingleResponse<UserResource>> {
        self.get("/identity").await
    }

    /// Fetches user identity with custom fields.
    ///
    /// # Parameters
    /// - `fields`: list of user fields to request
    ///
    /// # Required scopes
    /// - `identity`
    /// - `identity[email]` (if requesting the `email` field)
    pub async fn identity_with_fields(
        &self,
        fields: &[&str],
    ) -> Result<SingleResponse<UserResource>> {
        let fields_param = fields.join(",");
        self.get(&format!("/identity?fields[user]={}", fields_param))
            .await
    }

    /// Fetches user identity including memberships.
    ///
    /// Returns the user identity and all memberships where the user is a patron.
    ///
    /// # Required scopes
    /// - `identity`
    /// - `identity.memberships`
    pub async fn identity_with_memberships(&self) -> Result<SingleResponse<UserResource>> {
        self.get("/identity?include=memberships&fields[user]=email,first_name,full_name,image_url,last_name,vanity,url&fields[member]=campaign_lifetime_support_cents,currently_entitled_amount_cents,email,full_name,is_follower,last_charge_date,last_charge_status,lifetime_support_cents,next_charge_date,note,patron_status,pledge_relationship_start").await
    }

    /// Fetches user identity including memberships and the related campaigns.
    ///
    /// Returns the user identity, memberships, and the referenced campaign resources.
    ///
    /// # Required scopes
    /// - `identity`
    /// - `identity.memberships`
    pub async fn identity_with_memberships_and_campaign(
        &self,
    ) -> Result<SingleResponse<UserResource>> {
        self.get("/identity?include=memberships,memberships.campaign&fields[user]=email,first_name,full_name,image_url,last_name,vanity,url&fields[member]=currently_entitled_amount_cents,lifetime_support_cents,patron_status&fields[campaign]=creation_name,image_url,url,vanity").await
    }

    /// Fetches a full identity response.
    ///
    /// Includes many available fields and related resources.
    ///
    /// # Required scopes
    /// - `identity`
    /// - `identity[email]`
    /// - `identity.memberships`
    pub async fn identity_full(&self) -> Result<SingleResponse<UserResource>> {
        self.get("/identity?include=memberships,memberships.campaign,memberships.currently_entitled_tiers&fields[user]=about,created,email,first_name,full_name,hide_pledges,image_url,is_creator,is_email_verified,last_name,like_count,social_connections,thumb_url,url,vanity&fields[member]=campaign_lifetime_support_cents,currently_entitled_amount_cents,email,full_name,is_follower,last_charge_date,last_charge_status,lifetime_support_cents,next_charge_date,note,patron_status,pledge_relationship_start,will_pay_amount_cents&fields[campaign]=creation_name,image_url,url,vanity&fields[tier]=amount_cents,description,title,url").await
    }
}

/// Field names for `identity_with_fields`.
pub mod identity_fields {
    /// User bio.
    pub const ABOUT: &str = "about";
    /// Account creation time.
    pub const CREATED: &str = "created";
    /// Email.
    pub const EMAIL: &str = "email";
    /// First name.
    pub const FIRST_NAME: &str = "first_name";
    /// Full name.
    pub const FULL_NAME: &str = "full_name";
    /// Whether pledges are hidden.
    pub const HIDE_PLEDGES: &str = "hide_pledges";
    /// Avatar image URL.
    pub const IMAGE_URL: &str = "image_url";
    /// Whether the user is a creator.
    pub const IS_CREATOR: &str = "is_creator";
    /// Whether the email is verified.
    pub const IS_EMAIL_VERIFIED: &str = "is_email_verified";
    /// Last name.
    pub const LAST_NAME: &str = "last_name";
    /// Like count.
    pub const LIKE_COUNT: &str = "like_count";
    /// Social connections.
    pub const SOCIAL_CONNECTIONS: &str = "social_connections";
    /// Thumbnail URL.
    pub const THUMB_URL: &str = "thumb_url";
    /// Patreon profile URL.
    pub const URL: &str = "url";
    /// Vanity username.
    pub const VANITY: &str = "vanity";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_client() {
        let client = PatreonUserClient::new("test_token");
        assert_eq!(client.access_token, "test_token");
    }

    #[test]
    fn test_auth_headers() {
        let client = PatreonUserClient::new("test_token");
        let headers = client.auth_headers();
        assert!(headers.get(AUTHORIZATION).is_some());
        assert_eq!(
            headers.get(AUTHORIZATION).unwrap().to_str().unwrap(),
            "Bearer test_token"
        );
    }
}
