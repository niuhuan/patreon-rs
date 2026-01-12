//! Patreon API response types.
//!
//! JSON:API response wrappers.

use serde::{Deserialize, Serialize};
use super::serde_helpers::de_null_default;

/// JSON:API response wrapper.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<D> {
    /// Primary data.
    pub data: D,
    /// Included related resources.
    #[serde(default, deserialize_with = "de_null_default")]
    pub included: Vec<serde_json::Value>,
    /// Pagination links.
    #[serde(default, deserialize_with = "de_null_default")]
    pub links: PaginationLinks,
    /// Metadata.
    #[serde(default, deserialize_with = "de_null_default")]
    pub meta: serde_json::Value,
}

/// Pagination links.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaginationLinks {
    /// First page.
    #[serde(default, deserialize_with = "de_null_default")]
    pub first: String,
    /// Previous page.
    #[serde(default, deserialize_with = "de_null_default")]
    pub prev: String,
    /// Next page.
    #[serde(default, deserialize_with = "de_null_default")]
    pub next: String,
    /// Last page.
    #[serde(default, deserialize_with = "de_null_default")]
    pub last: String,
    /// Current page.
    #[serde(rename = "self", default, deserialize_with = "de_null_default")]
    pub self_link: String,
}

/// Pagination metadata.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaginationMeta {
    /// Total count.
    #[serde(default, deserialize_with = "de_null_default")]
    pub count: i32,
}

/// Single resource response.
pub type SingleResponse<T> = ApiResponse<T>;

/// List resource response.
pub type ListResponse<T> = ApiResponse<Vec<T>>;

/// API error response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    /// Error list.
    pub errors: Vec<ApiErrorDetail>,
}

/// API error detail.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiErrorDetail {
    /// Error code.
    #[serde(default, deserialize_with = "de_null_default")]
    pub code: i32,
    /// HTTP status code (string).
    #[serde(default, deserialize_with = "de_null_default")]
    pub status: String,
    /// Error title.
    #[serde(default, deserialize_with = "de_null_default")]
    pub title: String,
    /// Error detail.
    #[serde(default, deserialize_with = "de_null_default")]
    pub detail: String,
    /// Error code name.
    #[serde(default, deserialize_with = "de_null_default")]
    pub code_name: String,
    /// Error ID.
    #[serde(default, deserialize_with = "de_null_default")]
    pub id: String,
}
