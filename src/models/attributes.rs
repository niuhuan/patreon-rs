//! Patreon API resource attributes.
//!
//! Each resource type has a corresponding attributes struct.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use super::serde_helpers::{de_null_default, de_null_unix_epoch, unix_epoch};

// ============== User ==============

/// User attributes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAttributes {
    /// Email address.
    #[serde(default, deserialize_with = "de_null_default")]
    pub email: String,

    /// Full name.
    #[serde(default, deserialize_with = "de_null_default")]
    pub full_name: String,

    /// First name.
    #[serde(default, deserialize_with = "de_null_default")]
    pub first_name: String,

    /// Last name.
    #[serde(default, deserialize_with = "de_null_default")]
    pub last_name: String,

    /// Vanity username.
    #[serde(default, deserialize_with = "de_null_default")]
    pub vanity: String,

    /// Bio/about text.
    #[serde(default, deserialize_with = "de_null_default")]
    pub about: String,

    /// Avatar image URL.
    #[serde(default, deserialize_with = "de_null_default")]
    pub image_url: String,

    /// Thumbnail URL.
    #[serde(default, deserialize_with = "de_null_default")]
    pub thumb_url: String,

    /// Patreon profile URL.
    #[serde(default, deserialize_with = "de_null_default")]
    pub url: String,

    /// Whether the user is a creator.
    #[serde(default, deserialize_with = "de_null_default")]
    pub is_creator: bool,

    /// Whether the email is verified.
    #[serde(default, deserialize_with = "de_null_default")]
    pub is_email_verified: bool,

    /// Account creation time.
    #[serde(
        default = "unix_epoch",
        deserialize_with = "de_null_unix_epoch"
    )]
    pub created: DateTime<Utc>,

    /// Whether pledges are hidden.
    #[serde(default, deserialize_with = "de_null_default")]
    pub hide_pledges: bool,

    /// Like count.
    #[serde(default, deserialize_with = "de_null_default")]
    pub like_count: i32,

    /// Social connections.
    #[serde(default, deserialize_with = "de_null_default")]
    pub social_connections: serde_json::Value,
}

impl Default for UserAttributes {
    fn default() -> Self {
        Self {
            email: String::new(),
            full_name: String::new(),
            first_name: String::new(),
            last_name: String::new(),
            vanity: String::new(),
            about: String::new(),
            image_url: String::new(),
            thumb_url: String::new(),
            url: String::new(),
            is_creator: false,
            is_email_verified: false,
            created: unix_epoch(),
            hide_pledges: false,
            like_count: 0,
            social_connections: serde_json::Value::default(),
        }
    }
}

// ============== Campaign ==============

/// Campaign attributes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignAttributes {
    /// Campaign creation time.
    #[serde(
        default = "unix_epoch",
        deserialize_with = "de_null_unix_epoch"
    )]
    pub created_at: DateTime<Utc>,

    /// Creation name / what the creator makes.
    #[serde(default, deserialize_with = "de_null_default")]
    pub creation_name: String,

    /// Discord server ID.
    #[serde(default, deserialize_with = "de_null_default")]
    pub discord_server_id: String,

    /// Google Analytics ID
    #[serde(default, deserialize_with = "de_null_default")]
    pub google_analytics_id: String,

    /// Whether the campaign charges immediately.
    #[serde(default, deserialize_with = "de_null_default")]
    pub is_charged_immediately: bool,

    /// Whether the campaign charges monthly.
    #[serde(default, deserialize_with = "de_null_default")]
    pub is_monthly: bool,

    /// Whether the campaign is marked NSFW.
    #[serde(default, deserialize_with = "de_null_default")]
    pub is_nsfw: bool,

    /// Main image URL.
    #[serde(default, deserialize_with = "de_null_default")]
    pub image_url: String,

    /// Small main image URL.
    #[serde(default, deserialize_with = "de_null_default")]
    pub image_small_url: String,

    /// Cover photo URL.
    #[serde(default, deserialize_with = "de_null_default")]
    pub cover_photo_url: String,

    /// Cover photo URL sizes.
    #[serde(default, deserialize_with = "de_null_default")]
    pub cover_photo_url_sizes: serde_json::Value,

    /// Main video embed HTML.
    #[serde(default, deserialize_with = "de_null_default")]
    pub main_video_embed: String,

    /// Main video URL.
    #[serde(default, deserialize_with = "de_null_default")]
    pub main_video_url: String,

    /// Thanks video URL.
    #[serde(default, deserialize_with = "de_null_default")]
    pub thanks_video_url: String,

    /// Thanks message.
    #[serde(default, deserialize_with = "de_null_default")]
    pub thanks_msg: String,

    /// Thanks embed HTML.
    #[serde(default, deserialize_with = "de_null_default")]
    pub thanks_embed: String,

    /// One-liner.
    #[serde(default, deserialize_with = "de_null_default")]
    pub one_liner: String,

    /// Patron count.
    #[serde(default, deserialize_with = "de_null_default")]
    pub patron_count: i32,

    /// Paid member count.
    #[serde(default, deserialize_with = "de_null_default")]
    pub paid_member_count: i32,

    /// Pledge sum in cents.
    #[serde(default, deserialize_with = "de_null_default")]
    pub pledge_sum_cents: i32,

    /// Currency.
    #[serde(default, deserialize_with = "de_null_default")]
    pub pledge_sum_currency: String,

    /// Published at.
    #[serde(
        default = "unix_epoch",
        deserialize_with = "de_null_unix_epoch"
    )]
    pub published_at: DateTime<Utc>,

    /// Summary.
    #[serde(default, deserialize_with = "de_null_default")]
    pub summary: String,

    /// Campaign URL.
    #[serde(default, deserialize_with = "de_null_default")]
    pub url: String,

    /// Vanity.
    #[serde(default, deserialize_with = "de_null_default")]
    pub vanity: String,

    /// Pay-per name.
    #[serde(default, deserialize_with = "de_null_default")]
    pub pay_per_name: String,

    /// Whether the campaign is published.
    #[serde(default, deserialize_with = "de_null_default")]
    pub is_published: bool,

    /// Whether earnings are visible.
    #[serde(default, deserialize_with = "de_null_default")]
    pub show_earnings: bool,
}

impl Default for CampaignAttributes {
    fn default() -> Self {
        Self {
            created_at: unix_epoch(),
            creation_name: String::new(),
            discord_server_id: String::new(),
            google_analytics_id: String::new(),
            is_charged_immediately: false,
            is_monthly: false,
            is_nsfw: false,
            image_url: String::new(),
            image_small_url: String::new(),
            cover_photo_url: String::new(),
            cover_photo_url_sizes: serde_json::Value::default(),
            main_video_embed: String::new(),
            main_video_url: String::new(),
            thanks_video_url: String::new(),
            thanks_msg: String::new(),
            thanks_embed: String::new(),
            one_liner: String::new(),
            patron_count: 0,
            paid_member_count: 0,
            pledge_sum_cents: 0,
            pledge_sum_currency: String::new(),
            published_at: unix_epoch(),
            summary: String::new(),
            url: String::new(),
            vanity: String::new(),
            pay_per_name: String::new(),
            is_published: false,
            show_earnings: false,
        }
    }
}

// ============== Member ==============

/// Member attributes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberAttributes {
    /// Patron status.
    #[serde(default, deserialize_with = "de_null_default")]
    pub patron_status: PatronStatus,

    /// Whether this member is following.
    #[serde(default, deserialize_with = "de_null_default")]
    pub is_follower: bool,

    /// Full name.
    #[serde(default, deserialize_with = "de_null_default")]
    pub full_name: String,

    /// Email.
    #[serde(default, deserialize_with = "de_null_default")]
    pub email: String,

    /// Currently entitled amount (cents).
    #[serde(default, deserialize_with = "de_null_default")]
    pub currently_entitled_amount_cents: i32,

    /// Lifetime support (cents).
    #[serde(default, deserialize_with = "de_null_default")]
    pub lifetime_support_cents: i32,

    /// Last charge date.
    #[serde(
        default = "unix_epoch",
        deserialize_with = "de_null_unix_epoch"
    )]
    pub last_charge_date: DateTime<Utc>,

    /// Last charge status.
    #[serde(default, deserialize_with = "de_null_default")]
    pub last_charge_status: ChargeStatus,

    /// Next charge date.
    #[serde(
        default = "unix_epoch",
        deserialize_with = "de_null_unix_epoch"
    )]
    pub next_charge_date: DateTime<Utc>,

    /// Pledge relationship start.
    #[serde(
        default = "unix_epoch",
        deserialize_with = "de_null_unix_epoch"
    )]
    pub pledge_relationship_start: DateTime<Utc>,

    /// Note.
    #[serde(default, deserialize_with = "de_null_default")]
    pub note: String,

    /// Will pay amount (cents).
    #[serde(default, deserialize_with = "de_null_default")]
    pub will_pay_amount_cents: i32,

    /// Campaign currency.
    #[serde(default, deserialize_with = "de_null_default")]
    pub campaign_currency: String,

    /// Campaign lifetime support (cents).
    #[serde(default, deserialize_with = "de_null_default")]
    pub campaign_lifetime_support_cents: i32,

    /// Campaign pledge amount (cents).
    #[serde(default, deserialize_with = "de_null_default")]
    pub campaign_pledge_amount_cents: i32,
}

impl Default for MemberAttributes {
    fn default() -> Self {
        Self {
            patron_status: PatronStatus::default(),
            is_follower: false,
            full_name: String::new(),
            email: String::new(),
            currently_entitled_amount_cents: 0,
            lifetime_support_cents: 0,
            last_charge_date: unix_epoch(),
            last_charge_status: ChargeStatus::default(),
            next_charge_date: unix_epoch(),
            pledge_relationship_start: unix_epoch(),
            note: String::new(),
            will_pay_amount_cents: 0,
            campaign_currency: String::new(),
            campaign_lifetime_support_cents: 0,
            campaign_pledge_amount_cents: 0,
        }
    }
}

/// Patron status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PatronStatus {
    /// Active patron.
    ActivePatron,
    /// Declined patron.
    DeclinedPatron,
    /// Former patron.
    FormerPatron,
    /// Other/unknown status.
    #[serde(other)]
    Unknown,
}

impl Default for PatronStatus {
    fn default() -> Self {
        Self::Unknown
    }
}

/// Charge status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ChargeStatus {
    /// Paid.
    Paid,
    /// Declined.
    Declined,
    /// Deleted.
    Deleted,
    /// Pending.
    Pending,
    /// Refunded.
    Refunded,
    /// Fraud.
    Fraud,
    /// Other/unknown.
    #[serde(other)]
    Unknown,
}

impl Default for ChargeStatus {
    fn default() -> Self {
        Self::Unknown
    }
}

// ============== Tier ==============

/// Tier attributes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierAttributes {
    /// Tier amount (cents).
    #[serde(default, deserialize_with = "de_null_default")]
    pub amount_cents: i32,

    /// Created at.
    #[serde(
        default = "unix_epoch",
        deserialize_with = "de_null_unix_epoch"
    )]
    pub created_at: DateTime<Utc>,

    /// Description.
    #[serde(default, deserialize_with = "de_null_default")]
    pub description: String,

    /// Discord role IDs.
    #[serde(default, deserialize_with = "de_null_default")]
    pub discord_role_ids: Vec<String>,

    /// Edited at.
    #[serde(
        default = "unix_epoch",
        deserialize_with = "de_null_unix_epoch"
    )]
    pub edited_at: DateTime<Utc>,

    /// Image URL.
    #[serde(default, deserialize_with = "de_null_default")]
    pub image_url: String,

    /// Patron count.
    #[serde(default, deserialize_with = "de_null_default")]
    pub patron_count: i32,

    /// Post count.
    #[serde(default, deserialize_with = "de_null_default")]
    pub post_count: i32,

    /// Whether published.
    #[serde(default, deserialize_with = "de_null_default")]
    pub published: bool,

    /// Published at.
    #[serde(
        default = "unix_epoch",
        deserialize_with = "de_null_unix_epoch"
    )]
    pub published_at: DateTime<Utc>,

    /// Title.
    #[serde(default, deserialize_with = "de_null_default")]
    pub title: String,

    /// Unpublished at.
    #[serde(
        default = "unix_epoch",
        deserialize_with = "de_null_unix_epoch"
    )]
    pub unpublished_at: DateTime<Utc>,

    /// Tier URL.
    #[serde(default, deserialize_with = "de_null_default")]
    pub url: String,

    /// User limit.
    #[serde(default, deserialize_with = "de_null_default")]
    pub user_limit: i32,

    /// Remaining capacity.
    #[serde(default, deserialize_with = "de_null_default")]
    pub remaining: i32,
}

impl Default for TierAttributes {
    fn default() -> Self {
        Self {
            amount_cents: 0,
            created_at: unix_epoch(),
            description: String::new(),
            discord_role_ids: Vec::new(),
            edited_at: unix_epoch(),
            image_url: String::new(),
            patron_count: 0,
            post_count: 0,
            published: false,
            published_at: unix_epoch(),
            title: String::new(),
            unpublished_at: unix_epoch(),
            url: String::new(),
            user_limit: 0,
            remaining: 0,
        }
    }
}

// ============== Post ==============

/// Post attributes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostAttributes {
    /// Title.
    #[serde(default, deserialize_with = "de_null_default")]
    pub title: String,

    /// Content (HTML).
    #[serde(default, deserialize_with = "de_null_default")]
    pub content: String,

    /// Whether public.
    #[serde(default, deserialize_with = "de_null_default")]
    pub is_public: bool,

    /// Whether paid.
    #[serde(default, deserialize_with = "de_null_default")]
    pub is_paid: bool,

    /// Published at.
    #[serde(
        default = "unix_epoch",
        deserialize_with = "de_null_unix_epoch"
    )]
    pub published_at: DateTime<Utc>,

    /// Edited at.
    #[serde(
        default = "unix_epoch",
        deserialize_with = "de_null_unix_epoch"
    )]
    pub edited_at: DateTime<Utc>,

    /// Created at.
    #[serde(
        default = "unix_epoch",
        deserialize_with = "de_null_unix_epoch"
    )]
    pub created_at: DateTime<Utc>,

    /// Embed data.
    #[serde(default, deserialize_with = "de_null_default")]
    pub embed: serde_json::Value,

    /// Embed URL.
    #[serde(default, deserialize_with = "de_null_default")]
    pub embed_url: String,

    /// App ID.
    #[serde(default, deserialize_with = "de_null_default")]
    pub app_id: i64,

    /// App status.
    #[serde(default, deserialize_with = "de_null_default")]
    pub app_status: String,

    /// Image.
    #[serde(default, deserialize_with = "de_null_default")]
    pub image: serde_json::Value,

    /// Whether this is a teaser.
    #[serde(default, deserialize_with = "de_null_default")]
    pub is_teaser: bool,

    /// Teaser text.
    #[serde(default, deserialize_with = "de_null_default")]
    pub teaser_text: String,

    /// Like count.
    #[serde(default, deserialize_with = "de_null_default")]
    pub like_count: i32,

    /// Comment count.
    #[serde(default, deserialize_with = "de_null_default")]
    pub comment_count: i32,

    /// Post URL.
    #[serde(default, deserialize_with = "de_null_default")]
    pub url: String,

    /// Post type.
    #[serde(default, deserialize_with = "de_null_default")]
    pub post_type: String,

    /// Post file.
    #[serde(default, deserialize_with = "de_null_default")]
    pub post_file: serde_json::Value,

    /// Post metadata.
    #[serde(default, deserialize_with = "de_null_default")]
    pub post_metadata: serde_json::Value,

    /// Minimum cents pledged to view.
    #[serde(default, deserialize_with = "de_null_default")]
    pub min_cents_pledged_to_view: i32,

    /// Thumbnail URL.
    #[serde(default, deserialize_with = "de_null_default")]
    pub thumbnail_url: String,

    /// Thumbnail.
    #[serde(default, deserialize_with = "de_null_default")]
    pub thumbnail: serde_json::Value,
}

impl Default for PostAttributes {
    fn default() -> Self {
        Self {
            title: String::new(),
            content: String::new(),
            is_public: false,
            is_paid: false,
            published_at: unix_epoch(),
            edited_at: unix_epoch(),
            created_at: unix_epoch(),
            embed: serde_json::Value::default(),
            embed_url: String::new(),
            app_id: 0,
            app_status: String::new(),
            image: serde_json::Value::default(),
            is_teaser: false,
            teaser_text: String::new(),
            like_count: 0,
            comment_count: 0,
            url: String::new(),
            post_type: String::new(),
            post_file: serde_json::Value::default(),
            post_metadata: serde_json::Value::default(),
            min_cents_pledged_to_view: 0,
            thumbnail_url: String::new(),
            thumbnail: serde_json::Value::default(),
        }
    }
}

// ============== Benefit ==============

/// Benefit attributes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenefitAttributes {
    /// Title.
    #[serde(default, deserialize_with = "de_null_default")]
    pub title: String,

    /// Description.
    #[serde(default, deserialize_with = "de_null_default")]
    pub description: String,

    /// Benefit type.
    #[serde(default, deserialize_with = "de_null_default")]
    pub benefit_type: String,

    /// Rule type.
    #[serde(default, deserialize_with = "de_null_default")]
    pub rule_type: String,

    /// Created at.
    #[serde(
        default = "unix_epoch",
        deserialize_with = "de_null_unix_epoch"
    )]
    pub created_at: DateTime<Utc>,

    /// Whether published.
    #[serde(default, deserialize_with = "de_null_default")]
    pub is_published: bool,

    /// Whether deleted.
    #[serde(default, deserialize_with = "de_null_default")]
    pub is_deleted: bool,

    /// Whether deliverable.
    #[serde(default, deserialize_with = "de_null_default")]
    pub is_deliverable: bool,

    /// Deliverables due today count.
    #[serde(default, deserialize_with = "de_null_default")]
    pub deliverables_due_today_count: i32,

    /// Delivered deliverables count.
    #[serde(default, deserialize_with = "de_null_default")]
    pub delivered_deliverables_count: i32,

    /// Not delivered deliverables count.
    #[serde(default, deserialize_with = "de_null_default")]
    pub not_delivered_deliverables_count: i32,

    /// Next deliverable due date.
    #[serde(
        default = "unix_epoch",
        deserialize_with = "de_null_unix_epoch"
    )]
    pub next_deliverable_due_date: DateTime<Utc>,

    /// Tiers count.
    #[serde(default, deserialize_with = "de_null_default")]
    pub tiers_count: i32,

    /// App external ID.
    #[serde(default, deserialize_with = "de_null_default")]
    pub app_external_id: String,

    /// App metadata.
    #[serde(default, deserialize_with = "de_null_default")]
    pub app_meta: serde_json::Value,
}

impl Default for BenefitAttributes {
    fn default() -> Self {
        Self {
            title: String::new(),
            description: String::new(),
            benefit_type: String::new(),
            rule_type: String::new(),
            created_at: unix_epoch(),
            is_published: false,
            is_deleted: false,
            is_deliverable: false,
            deliverables_due_today_count: 0,
            delivered_deliverables_count: 0,
            not_delivered_deliverables_count: 0,
            next_deliverable_due_date: unix_epoch(),
            tiers_count: 0,
            app_external_id: String::new(),
            app_meta: serde_json::Value::default(),
        }
    }
}

// ============== Address ==============

/// Address attributes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressAttributes {
    /// Addressee.
    #[serde(default, deserialize_with = "de_null_default")]
    pub addressee: String,

    /// City.
    #[serde(default, deserialize_with = "de_null_default")]
    pub city: String,

    /// Country.
    #[serde(default, deserialize_with = "de_null_default")]
    pub country: String,

    /// Created at.
    #[serde(
        default = "unix_epoch",
        deserialize_with = "de_null_unix_epoch"
    )]
    pub created_at: DateTime<Utc>,

    /// Line 1.
    #[serde(default, deserialize_with = "de_null_default")]
    pub line_1: String,

    /// Line 2.
    #[serde(default, deserialize_with = "de_null_default")]
    pub line_2: String,

    /// Phone number.
    #[serde(default, deserialize_with = "de_null_default")]
    pub phone_number: String,

    /// Postal code.
    #[serde(default, deserialize_with = "de_null_default")]
    pub postal_code: String,

    /// State/region.
    #[serde(default, deserialize_with = "de_null_default")]
    pub state: String,

    /// Whether confirmed.
    #[serde(default, deserialize_with = "de_null_default")]
    pub confirmed: bool,

    /// Confirmed at.
    #[serde(
        default = "unix_epoch",
        deserialize_with = "de_null_unix_epoch"
    )]
    pub confirmed_at: DateTime<Utc>,
}

impl Default for AddressAttributes {
    fn default() -> Self {
        Self {
            addressee: String::new(),
            city: String::new(),
            country: String::new(),
            created_at: unix_epoch(),
            line_1: String::new(),
            line_2: String::new(),
            phone_number: String::new(),
            postal_code: String::new(),
            state: String::new(),
            confirmed: false,
            confirmed_at: unix_epoch(),
        }
    }
}

// ============== Goal ==============

/// Goal attributes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalAttributes {
    /// Amount (cents).
    #[serde(default, deserialize_with = "de_null_default")]
    pub amount_cents: i32,

    /// Completed percentage.
    #[serde(default, deserialize_with = "de_null_default")]
    pub completed_percentage: i32,

    /// Created at.
    #[serde(
        default = "unix_epoch",
        deserialize_with = "de_null_unix_epoch"
    )]
    pub created_at: DateTime<Utc>,

    /// Description.
    #[serde(default, deserialize_with = "de_null_default")]
    pub description: String,

    /// Reached at.
    #[serde(
        default = "unix_epoch",
        deserialize_with = "de_null_unix_epoch"
    )]
    pub reached_at: DateTime<Utc>,

    /// Title.
    #[serde(default, deserialize_with = "de_null_default")]
    pub title: String,
}

impl Default for GoalAttributes {
    fn default() -> Self {
        Self {
            amount_cents: 0,
            completed_percentage: 0,
            created_at: unix_epoch(),
            description: String::new(),
            reached_at: unix_epoch(),
            title: String::new(),
        }
    }
}

// ============== Media ==============

/// Media attributes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaAttributes {
    /// Created at.
    #[serde(
        default = "unix_epoch",
        deserialize_with = "de_null_unix_epoch"
    )]
    pub created_at: DateTime<Utc>,

    /// Download URL.
    #[serde(default, deserialize_with = "de_null_default")]
    pub download_url: String,

    /// File name.
    #[serde(default, deserialize_with = "de_null_default")]
    pub file_name: String,

    /// Image URLs.
    #[serde(default, deserialize_with = "de_null_default")]
    pub image_urls: serde_json::Value,

    /// Metadata.
    #[serde(default, deserialize_with = "de_null_default")]
    pub metadata: serde_json::Value,

    /// MIME type.
    #[serde(default, deserialize_with = "de_null_default")]
    pub mimetype: String,

    /// Owner ID.
    #[serde(default, deserialize_with = "de_null_default")]
    pub owner_id: String,

    /// Owner relationship.
    #[serde(default, deserialize_with = "de_null_default")]
    pub owner_relationship: String,

    /// Owner type.
    #[serde(default, deserialize_with = "de_null_default")]
    pub owner_type: String,

    /// Size in bytes.
    #[serde(default, deserialize_with = "de_null_default")]
    pub size_bytes: i64,

    /// State.
    #[serde(default, deserialize_with = "de_null_default")]
    pub state: String,

    /// Upload expires at.
    #[serde(
        default = "unix_epoch",
        deserialize_with = "de_null_unix_epoch"
    )]
    pub upload_expires_at: DateTime<Utc>,

    /// Upload parameters.
    #[serde(default, deserialize_with = "de_null_default")]
    pub upload_parameters: serde_json::Value,

    /// Upload URL.
    #[serde(default, deserialize_with = "de_null_default")]
    pub upload_url: String,
}

impl Default for MediaAttributes {
    fn default() -> Self {
        Self {
            created_at: unix_epoch(),
            download_url: String::new(),
            file_name: String::new(),
            image_urls: serde_json::Value::default(),
            metadata: serde_json::Value::default(),
            mimetype: String::new(),
            owner_id: String::new(),
            owner_relationship: String::new(),
            owner_type: String::new(),
            size_bytes: 0,
            state: String::new(),
            upload_expires_at: unix_epoch(),
            upload_parameters: serde_json::Value::default(),
            upload_url: String::new(),
        }
    }
}

// ============== Webhook ==============

/// Webhook attributes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookAttributes {
    /// Last attempted at.
    #[serde(
        default = "unix_epoch",
        deserialize_with = "de_null_unix_epoch"
    )]
    pub last_attempted_at: DateTime<Utc>,

    /// Consecutive failure count.
    #[serde(default, deserialize_with = "de_null_default")]
    pub num_consecutive_times_failed: i32,

    /// Whether paused.
    #[serde(default, deserialize_with = "de_null_default")]
    pub paused: bool,

    /// Secret.
    #[serde(default, deserialize_with = "de_null_default")]
    pub secret: String,

    /// Trigger list.
    #[serde(default, deserialize_with = "de_null_default")]
    pub triggers: Vec<WebhookTrigger>,

    /// Webhook URL.
    #[serde(default, deserialize_with = "de_null_default")]
    pub uri: String,
}

impl Default for WebhookAttributes {
    fn default() -> Self {
        Self {
            last_attempted_at: unix_epoch(),
            num_consecutive_times_failed: 0,
            paused: false,
            secret: String::new(),
            triggers: Vec::new(),
            uri: String::new(),
        }
    }
}

/// Webhook trigger type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum WebhookTrigger {
    /// Member created.
    #[serde(rename = "members:create")]
    MembersCreate,

    /// Member updated.
    #[serde(rename = "members:update")]
    MembersUpdate,

    /// Member deleted.
    #[serde(rename = "members:delete")]
    MembersDelete,

    /// Pledge created.
    #[serde(rename = "members:pledge:create")]
    MembersPledgeCreate,

    /// Pledge updated.
    #[serde(rename = "members:pledge:update")]
    MembersPledgeUpdate,

    /// Pledge deleted.
    #[serde(rename = "members:pledge:delete")]
    MembersPledgeDelete,

    /// Post published.
    #[serde(rename = "posts:publish")]
    PostsPublish,

    /// Post updated.
    #[serde(rename = "posts:update")]
    PostsUpdate,

    /// Post deleted.
    #[serde(rename = "posts:delete")]
    PostsDelete,

    /// Other/unknown trigger.
    #[serde(other)]
    Unknown,
}

impl Default for WebhookTrigger {
    fn default() -> Self {
        Self::Unknown
    }
}
