//! Server API client (creator client).
//!
//! Used by creators to access their campaigns, members, posts, and webhooks.
//! These APIs require a creator `access_token` (Creator's Access Token from Patreon developer portal).
//!
//! ## Typical use cases
//! - Fetch campaigns on the server side
//! - List members/patrons
//! - Manage posts
//! - Handle webhooks
//!
//! ## Available APIs
//! - `/api/oauth2/v2/campaigns` - list all campaigns
//! - `/api/oauth2/v2/campaigns/{campaign_id}` - fetch campaign details
//! - `/api/oauth2/v2/campaigns/{campaign_id}/members` - list members
//! - `/api/oauth2/v2/campaigns/{campaign_id}/posts` - list posts
//! - `/api/oauth2/v2/members/{member_id}` - fetch a member
//! - `/api/oauth2/v2/posts/{post_id}` - fetch a post
//! - `/api/oauth2/v2/webhooks` - manage webhooks

use crate::models::*;
use crate::{Error, Result, API_BASE_URL};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::Serialize;

/// Patreon creator (server) API client.
///
/// Used by creators to access their campaigns, members, posts, and webhooks.
///
/// # Example
///
/// ```rust,ignore
/// use patreon::PatreonCreatorClient;
///
/// // Using a creator access token
/// let client = PatreonCreatorClient::new("creator_access_token");
///
/// // List campaigns
/// let campaigns = client.campaigns().await?;
///
/// // List members for a campaign
/// let members = client.campaign_members("campaign_id").await?;
/// ```
#[derive(Debug, Clone)]
pub struct PatreonCreatorClient {
    access_token: String,
    http_client: reqwest::Client,
    base_url: String,
}

/// Query parameters for listing members.
#[derive(Debug, Clone, Default)]
pub struct MembersQuery {
    /// Cursor (for pagination).
    pub cursor: Option<String>,
    /// Page size (max 1000).
    pub page_size: Option<u32>,
}

/// Query parameters for listing posts.
#[derive(Debug, Clone, Default)]
pub struct PostsQuery {
    /// Cursor (for pagination).
    pub cursor: Option<String>,
    /// Page size (max 100).
    pub page_size: Option<u32>,
}

/// Parameters for creating a webhook.
#[derive(Debug, Clone, Serialize)]
pub struct CreateWebhookRequest {
    /// Webhook URL.
    pub uri: String,
    /// Campaign ID.
    pub campaign_id: String,
    /// Trigger list.
    pub triggers: Vec<String>,
}

/// Webhook request body (JSON:API format).
#[derive(Debug, Clone, Serialize)]
struct WebhookRequestBody {
    data: WebhookRequestData,
}

#[derive(Debug, Clone, Serialize)]
struct WebhookRequestData {
    #[serde(rename = "type")]
    resource_type: String,
    attributes: WebhookRequestAttributes,
    relationships: WebhookRequestRelationships,
}

#[derive(Debug, Clone, Serialize)]
struct WebhookRequestAttributes {
    uri: String,
    triggers: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct WebhookRequestRelationships {
    campaign: WebhookCampaignRelationship,
}

#[derive(Debug, Clone, Serialize)]
struct WebhookCampaignRelationship {
    data: WebhookCampaignData,
}

#[derive(Debug, Clone, Serialize)]
struct WebhookCampaignData {
    #[serde(rename = "type")]
    resource_type: String,
    id: String,
}

impl PatreonCreatorClient {
    /// Creates a new creator client.
    ///
    /// # Parameters
    /// - `access_token`: creator access token
    ///   (available from <https://www.patreon.com/portal/registration/register-clients>)
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

    /// Sends a POST request.
    async fn post_request<T: serde::de::DeserializeOwned, B: Serialize>(
        &self,
        endpoint: &str,
        body: &B,
    ) -> Result<T> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self
            .http_client
            .post(&url)
            .headers(self.auth_headers())
            .json(body)
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

    /// Sends a PATCH request.
    async fn patch<T: serde::de::DeserializeOwned, B: Serialize>(
        &self,
        endpoint: &str,
        body: &B,
    ) -> Result<T> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self
            .http_client
            .patch(&url)
            .headers(self.auth_headers())
            .json(body)
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

    /// Sends a DELETE request.
    async fn delete(&self, endpoint: &str) -> Result<()> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self
            .http_client
            .delete(&url)
            .headers(self.auth_headers())
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status().as_u16();
            let text = response.text().await.unwrap_or_default();
            Err(Error::Api {
                status,
                message: text,
            })
        }
    }

    // ==================== Campaigns API ====================

    /// Lists all campaigns owned by the current creator.
    ///
    /// # Required scopes
    /// - `campaigns`
    pub async fn campaigns(&self) -> Result<ListResponse<CampaignResource>> {
        self.get("/campaigns").await
    }

    /// Lists campaigns with details.
    ///
    /// Returns detailed campaign information including creator info.
    pub async fn campaigns_with_details(&self) -> Result<ListResponse<CampaignResource>> {
        self.get("/campaigns?include=creator&fields[campaign]=created_at,creation_name,discord_server_id,image_url,image_small_url,is_charged_immediately,is_monthly,is_nsfw,main_video_embed,main_video_url,one_liner,patron_count,pay_per_name,pledge_sum_cents,pledge_sum_currency,published_at,summary,thanks_embed,thanks_msg,thanks_video_url,url,vanity&fields[user]=full_name,image_url,url").await
    }

    /// Fetches a specific campaign.
    ///
    /// # Parameters
    /// - `campaign_id`: campaign ID
    pub async fn campaign(&self, campaign_id: &str) -> Result<SingleResponse<CampaignResource>> {
        self.get(&format!("/campaigns/{}", campaign_id)).await
    }

    /// Fetches a campaign including tiers and benefits.
    ///
    /// # Parameters
    /// - `campaign_id`: campaign ID
    pub async fn campaign_with_tiers_and_benefits(
        &self,
        campaign_id: &str,
    ) -> Result<SingleResponse<CampaignResource>> {
        self.get(&format!("/campaigns/{}?include=tiers,tiers.benefits,creator,goals&fields[campaign]=created_at,creation_name,discord_server_id,image_url,image_small_url,is_charged_immediately,is_monthly,is_nsfw,main_video_embed,main_video_url,one_liner,patron_count,pay_per_name,pledge_sum_cents,pledge_sum_currency,published_at,summary,thanks_embed,thanks_msg,thanks_video_url,url,vanity,show_earnings&fields[tier]=amount_cents,created_at,description,discord_role_ids,edited_at,image_url,patron_count,post_count,published,published_at,title,unpublished_at,url,user_limit&fields[benefit]=benefit_type,created_at,deliverables_due_today_count,delivered_deliverables_count,description,is_deleted,is_published,next_deliverable_due_date,not_delivered_deliverables_count,rule_type,tiers_count,title&fields[goal]=amount_cents,completed_percentage,created_at,description,reached_at,title&fields[user]=full_name,image_url,url", campaign_id)).await
    }

    // ==================== Members API ====================

    /// Lists all members for a campaign.
    ///
    /// # Parameters
    /// - `campaign_id`: campaign ID
    ///
    /// # Required scopes
    /// - `campaigns.members`
    pub async fn campaign_members(
        &self,
        campaign_id: &str,
    ) -> Result<ListResponse<MemberResource>> {
        self.get(&format!("/campaigns/{}/members", campaign_id))
            .await
    }

    /// Lists campaign members with pagination parameters.
    ///
    /// # Parameters
    /// - `campaign_id`: campaign ID
    /// - `query`: query parameters
    pub async fn campaign_members_with_query(
        &self,
        campaign_id: &str,
        query: &MembersQuery,
    ) -> Result<ListResponse<MemberResource>> {
        let mut endpoint = format!("/campaigns/{}/members?", campaign_id);

        if let Some(ref cursor) = query.cursor {
            endpoint.push_str(&format!("page[cursor]={}&", cursor));
        }
        if let Some(page_size) = query.page_size {
            endpoint.push_str(&format!("page[count]={}&", page_size.min(1000)));
        }

        self.get(&endpoint).await
    }

    /// Lists campaign members including related resources.
    ///
    /// Includes user info, currently entitled tiers, and address (if available).
    pub async fn campaign_members_with_details(
        &self,
        campaign_id: &str,
    ) -> Result<ListResponse<MemberResource>> {
        self.get(&format!("/campaigns/{}/members?include=user,currently_entitled_tiers,address&fields[member]=campaign_lifetime_support_cents,currently_entitled_amount_cents,email,full_name,is_follower,last_charge_date,last_charge_status,lifetime_support_cents,next_charge_date,note,patron_status,pledge_relationship_start,will_pay_amount_cents&fields[user]=email,full_name,image_url,url,vanity&fields[tier]=amount_cents,title,url&fields[address]=addressee,city,country,line_1,line_2,phone_number,postal_code,state", campaign_id)).await
    }

    /// Lists campaign members including related resources with pagination.
    pub async fn campaign_members_with_details_and_query(
        &self,
        campaign_id: &str,
        query: &MembersQuery,
    ) -> Result<ListResponse<MemberResource>> {
        let mut endpoint = format!("/campaigns/{}/members?include=user,currently_entitled_tiers,address&fields[member]=campaign_lifetime_support_cents,currently_entitled_amount_cents,email,full_name,is_follower,last_charge_date,last_charge_status,lifetime_support_cents,next_charge_date,note,patron_status,pledge_relationship_start,will_pay_amount_cents&fields[user]=email,full_name,image_url,url,vanity&fields[tier]=amount_cents,title,url&fields[address]=addressee,city,country,line_1,line_2,phone_number,postal_code,state", campaign_id);

        if let Some(ref cursor) = query.cursor {
            endpoint.push_str(&format!("&page[cursor]={}", cursor));
        }
        if let Some(page_size) = query.page_size {
            endpoint.push_str(&format!("&page[count]={}", page_size.min(1000)));
        }

        self.get(&endpoint).await
    }

    /// Fetches a specific member.
    ///
    /// # Parameters
    /// - `member_id`: member ID
    pub async fn member(&self, member_id: &str) -> Result<SingleResponse<MemberResource>> {
        self.get(&format!("/members/{}", member_id)).await
    }

    /// Fetches a member including related resources.
    pub async fn member_with_details(
        &self,
        member_id: &str,
    ) -> Result<SingleResponse<MemberResource>> {
        self.get(&format!("/members/{}?include=user,currently_entitled_tiers,address,campaign&fields[member]=campaign_lifetime_support_cents,currently_entitled_amount_cents,email,full_name,is_follower,last_charge_date,last_charge_status,lifetime_support_cents,next_charge_date,note,patron_status,pledge_relationship_start,will_pay_amount_cents&fields[user]=email,full_name,image_url,url,vanity&fields[tier]=amount_cents,title,url&fields[address]=addressee,city,country,line_1,line_2,phone_number,postal_code,state&fields[campaign]=creation_name,image_url,url,vanity", member_id)).await
    }

    // ==================== Posts API ====================

    /// Lists all posts for a campaign.
    ///
    /// # Parameters
    /// - `campaign_id`: campaign ID
    ///
    /// # Required scopes
    /// - `campaigns.posts`
    pub async fn campaign_posts(&self, campaign_id: &str) -> Result<ListResponse<PostResource>> {
        self.get(&format!("/campaigns/{}/posts", campaign_id)).await
    }

    /// Lists campaign posts with pagination parameters.
    pub async fn campaign_posts_with_query(
        &self,
        campaign_id: &str,
        query: &PostsQuery,
    ) -> Result<ListResponse<PostResource>> {
        let mut endpoint = format!("/campaigns/{}/posts?", campaign_id);

        if let Some(ref cursor) = query.cursor {
            endpoint.push_str(&format!("page[cursor]={}&", cursor));
        }
        if let Some(page_size) = query.page_size {
            endpoint.push_str(&format!("page[count]={}&", page_size.min(100)));
        }

        self.get(&endpoint).await
    }

    /// Lists campaign posts including related resources.
    pub async fn campaign_posts_with_details(
        &self,
        campaign_id: &str,
    ) -> Result<ListResponse<PostResource>> {
        self.get(&format!("/campaigns/{}/posts?include=user,campaign&fields[post]=app_id,app_status,content,embed_data,embed_url,is_paid,is_public,published_at,title,url,was_posted_by_campaign_owner,comment_count,like_count,teaser_text&fields[user]=full_name,image_url,url,vanity&fields[campaign]=creation_name,url,vanity", campaign_id)).await
    }

    /// Fetches a specific post.
    ///
    /// # Parameters
    /// - `post_id`: post ID
    pub async fn post(&self, post_id: &str) -> Result<SingleResponse<PostResource>> {
        self.get(&format!("/posts/{}", post_id)).await
    }

    /// Fetches a post including related resources.
    pub async fn post_with_details(
        &self,
        post_id: &str,
    ) -> Result<SingleResponse<PostResource>> {
        self.get(&format!("/posts/{}?include=user,campaign&fields[post]=app_id,app_status,content,embed_data,embed_url,is_paid,is_public,published_at,title,url,was_posted_by_campaign_owner,comment_count,like_count,teaser_text&fields[user]=full_name,image_url,url,vanity&fields[campaign]=creation_name,url,vanity", post_id)).await
    }

    // ==================== Webhooks API ====================

    /// Lists all webhooks.
    ///
    /// # Required scopes
    /// - `w:campaigns.webhook`
    pub async fn webhooks(&self) -> Result<ListResponse<WebhookResource>> {
        self.get("/webhooks").await
    }

    /// Creates a webhook.
    ///
    /// # Parameters
    /// - `request`: webhook creation parameters
    ///
    /// # Trigger types
    /// - `members:create` - member created
    /// - `members:update` - member updated
    /// - `members:delete` - member deleted
    /// - `members:pledge:create` - pledge created
    /// - `members:pledge:update` - pledge updated
    /// - `members:pledge:delete` - pledge deleted
    /// - `posts:publish` - post published
    /// - `posts:update` - post updated
    /// - `posts:delete` - post deleted
    pub async fn create_webhook(
        &self,
        request: &CreateWebhookRequest,
    ) -> Result<SingleResponse<WebhookResource>> {
        let body = WebhookRequestBody {
            data: WebhookRequestData {
                resource_type: "webhook".to_string(),
                attributes: WebhookRequestAttributes {
                    uri: request.uri.clone(),
                    triggers: request.triggers.clone(),
                },
                relationships: WebhookRequestRelationships {
                    campaign: WebhookCampaignRelationship {
                        data: WebhookCampaignData {
                            resource_type: "campaign".to_string(),
                            id: request.campaign_id.clone(),
                        },
                    },
                },
            },
        };

        self.post_request("/webhooks", &body).await
    }

    /// Updates a webhook.
    ///
    /// # Parameters
    /// - `webhook_id`: webhook ID
    /// - `uri`: new webhook URL (optional)
    /// - `triggers`: new trigger list (optional)
    /// - `paused`: whether the webhook is paused (optional)
    pub async fn update_webhook(
        &self,
        webhook_id: &str,
        uri: Option<&str>,
        triggers: Option<&[&str]>,
        paused: Option<bool>,
    ) -> Result<SingleResponse<WebhookResource>> {
        #[derive(Serialize)]
        struct UpdateBody {
            data: UpdateData,
        }

        #[derive(Serialize)]
        struct UpdateData {
            #[serde(rename = "type")]
            resource_type: String,
            id: String,
            attributes: UpdateAttributes,
        }

        #[derive(Serialize)]
        struct UpdateAttributes {
            #[serde(skip_serializing_if = "Option::is_none")]
            uri: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            triggers: Option<Vec<String>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            paused: Option<bool>,
        }

        let body = UpdateBody {
            data: UpdateData {
                resource_type: "webhook".to_string(),
                id: webhook_id.to_string(),
                attributes: UpdateAttributes {
                    uri: uri.map(String::from),
                    triggers: triggers.map(|t| t.iter().map(|s| s.to_string()).collect()),
                    paused,
                },
            },
        };

        self.patch(&format!("/webhooks/{}", webhook_id), &body).await
    }

    /// Deletes a webhook.
    ///
    /// # Parameters
    /// - `webhook_id`: webhook ID
    pub async fn delete_webhook(&self, webhook_id: &str) -> Result<()> {
        self.delete(&format!("/webhooks/{}", webhook_id)).await
    }
}

/// Field names for campaign resources.
pub mod campaign_fields {
    pub const CREATED_AT: &str = "created_at";
    pub const CREATION_NAME: &str = "creation_name";
    pub const DISCORD_SERVER_ID: &str = "discord_server_id";
    pub const GOOGLE_ANALYTICS_ID: &str = "google_analytics_id";
    pub const IMAGE_URL: &str = "image_url";
    pub const IMAGE_SMALL_URL: &str = "image_small_url";
    pub const IS_CHARGED_IMMEDIATELY: &str = "is_charged_immediately";
    pub const IS_MONTHLY: &str = "is_monthly";
    pub const IS_NSFW: &str = "is_nsfw";
    pub const MAIN_VIDEO_EMBED: &str = "main_video_embed";
    pub const MAIN_VIDEO_URL: &str = "main_video_url";
    pub const ONE_LINER: &str = "one_liner";
    pub const PATRON_COUNT: &str = "patron_count";
    pub const PAY_PER_NAME: &str = "pay_per_name";
    pub const PLEDGE_SUM_CENTS: &str = "pledge_sum_cents";
    pub const PLEDGE_SUM_CURRENCY: &str = "pledge_sum_currency";
    pub const PUBLISHED_AT: &str = "published_at";
    pub const SUMMARY: &str = "summary";
    pub const THANKS_EMBED: &str = "thanks_embed";
    pub const THANKS_MSG: &str = "thanks_msg";
    pub const THANKS_VIDEO_URL: &str = "thanks_video_url";
    pub const URL: &str = "url";
    pub const VANITY: &str = "vanity";
}

/// Field names for member resources.
pub mod member_fields {
    pub const CAMPAIGN_LIFETIME_SUPPORT_CENTS: &str = "campaign_lifetime_support_cents";
    pub const CURRENTLY_ENTITLED_AMOUNT_CENTS: &str = "currently_entitled_amount_cents";
    pub const EMAIL: &str = "email";
    pub const FULL_NAME: &str = "full_name";
    pub const IS_FOLLOWER: &str = "is_follower";
    pub const LAST_CHARGE_DATE: &str = "last_charge_date";
    pub const LAST_CHARGE_STATUS: &str = "last_charge_status";
    pub const LIFETIME_SUPPORT_CENTS: &str = "lifetime_support_cents";
    pub const NEXT_CHARGE_DATE: &str = "next_charge_date";
    pub const NOTE: &str = "note";
    pub const PATRON_STATUS: &str = "patron_status";
    pub const PLEDGE_RELATIONSHIP_START: &str = "pledge_relationship_start";
    pub const WILL_PAY_AMOUNT_CENTS: &str = "will_pay_amount_cents";
}

/// Webhook trigger string constants.
pub mod webhook_triggers {
    pub const MEMBERS_CREATE: &str = "members:create";
    pub const MEMBERS_UPDATE: &str = "members:update";
    pub const MEMBERS_DELETE: &str = "members:delete";
    pub const MEMBERS_PLEDGE_CREATE: &str = "members:pledge:create";
    pub const MEMBERS_PLEDGE_UPDATE: &str = "members:pledge:update";
    pub const MEMBERS_PLEDGE_DELETE: &str = "members:pledge:delete";
    pub const POSTS_PUBLISH: &str = "posts:publish";
    pub const POSTS_UPDATE: &str = "posts:update";
    pub const POSTS_DELETE: &str = "posts:delete";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creator_client() {
        let client = PatreonCreatorClient::new("test_token");
        assert_eq!(client.access_token, "test_token");
    }
}
