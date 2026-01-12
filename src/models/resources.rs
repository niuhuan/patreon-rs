//! Patreon API resource types.
//!
//! Defines resource types returned by the Patreon API.

use serde::{Deserialize, Serialize};
use super::attributes::*;

/// Resource type enum.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ResourceType {
    User,
    Campaign,
    Member,
    Tier,
    Post,
    Benefit,
    Deliverable,
    Address,
    Goal,
    Media,
    Webhook,
    PledgeEvent,
    #[serde(other)]
    Unknown,
}

/// JSON:API resource object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource<A> {
    /// Resource ID.
    pub id: String,
    /// Resource type.
    #[serde(rename = "type")]
    pub resource_type: ResourceType,
    /// Resource attributes.
    #[serde(default)]
    pub attributes: Option<A>,
    /// Relationships.
    #[serde(default)]
    pub relationships: Option<serde_json::Value>,
}

/// Resource reference (as used in relationships).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRef {
    /// Resource ID.
    pub id: String,
    /// Resource type.
    #[serde(rename = "type")]
    pub resource_type: ResourceType,
}

/// Relationship data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipData {
    /// Relationship data (single or multiple).
    #[serde(default)]
    pub data: Option<RelationshipDataValue>,
    /// Relationship links.
    #[serde(default)]
    pub links: Option<serde_json::Value>,
}

/// Relationship data value.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RelationshipDataValue {
    /// Single resource reference.
    Single(ResourceRef),
    /// Multiple resource references.
    Multiple(Vec<ResourceRef>),
}

// ============== Type aliases ==============

/// User resource.
pub type UserResource = Resource<UserAttributes>;

/// Campaign resource.
pub type CampaignResource = Resource<CampaignAttributes>;

/// Member resource.
pub type MemberResource = Resource<MemberAttributes>;

/// Tier resource.
pub type TierResource = Resource<TierAttributes>;

/// Post resource.
pub type PostResource = Resource<PostAttributes>;

/// Benefit resource.
pub type BenefitResource = Resource<BenefitAttributes>;

/// Address resource.
pub type AddressResource = Resource<AddressAttributes>;

/// Goal resource.
pub type GoalResource = Resource<GoalAttributes>;

/// Media resource.
pub type MediaResource = Resource<MediaAttributes>;

/// Webhook resource.
pub type WebhookResource = Resource<WebhookAttributes>;
