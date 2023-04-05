use crate::{ApiError, PatreonError, PatreonResult};
use chrono::{DateTime, Utc};
use reqwest::StatusCode;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::sync::Arc;
use url::Url;

static BASE_URI: &str = "https://www.patreon.com";

#[derive(Debug, Default)]
pub struct PatreonApi {
    pub access_token: String,
    pub agent: Arc<reqwest::Client>,
}

impl PatreonApi {
    pub async fn current_user(&self) -> PatreonResult<User> {
        let mut url = Url::parse(BASE_URI).unwrap();
        url.set_path("/api/oauth2/api/current_user");
        let response = self
            .agent
            .get(url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .header("User-Agent", "Patreon-rust")
            .send()
            .await?;
        let status = response.status();
        let text = response.text().await?;
        de_response(status, text)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct DocResponse<D> {
    data: D,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiDocument<A> {
    #[serde(rename = "type")]
    pub document_type: String,
    pub id: String,
    pub attributes: A,
}

pub type User = ApiDocument<UserAttributes>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserAttributes {
    pub first_name: String,
    pub last_name: String,
    pub full_name: String,
    pub vanity: Option<String>,
    pub email: String,
    pub about: Option<String>,
    pub facebook_id: Option<String>,
    pub image_url: String,
    pub thumb_url: String,
    pub youtube: Option<String>,
    pub twitter: Option<String>,
    pub facebook: Option<String>,
    pub created: DateTime<Utc>,
    pub url: String,
}

#[derive(Serialize, Deserialize)]
struct ApiErrorResponse {
    pub errors: Vec<ApiError>,
}

fn de_response<T: for<'de> serde::Deserialize<'de>>(
    status: StatusCode,
    text: String,
) -> PatreonResult<T> {
    if status.is_success() {
        Ok(serde_json::from_str::<DocResponse<T>>(text.as_str())?.data)
    } else {
        Err(PatreonError::PatreonApi(
            status,
            serde_json::from_str::<ApiErrorResponse>(text.as_str())?.errors,
        ))
    }
}
