use crate::{PatreonError, PatreonResult};
use reqwest::StatusCode;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use url::Url;

static BASE_URI: &str = "https://www.patreon.com";

#[derive(Debug, Default)]
pub struct PatreonOAuth {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub agent: Arc<reqwest::Client>,
}

impl PatreonOAuth {
    pub fn get_authorization_url(&self, scope: &str, state: &str) -> String {
        let mut url = Url::parse(BASE_URI).unwrap();
        url.set_path("/oauth2/authorize");
        if !scope.is_empty() {
            url.query_pairs_mut().append_pair("scope", scope);
        }
        if !state.is_empty() {
            url.query_pairs_mut().append_pair("state", state);
        }
        url.query_pairs_mut()
            .append_pair("scope ", "campaigns")
            .append_pair("response_type", "code")
            .append_pair("client_id", self.client_id.as_str())
            .append_pair("redirect_uri", self.redirect_uri.as_str());
        url.to_string()
    }

    pub async fn get_tokens(&self, code: &str) -> PatreonResult<TokensResponse> {
        self.parse_token_request(&{
            let mut params = HashMap::new();
            params.insert("grant_type", "authorization_code");
            params.insert("code", code);
            params.insert("client_id", self.client_id.as_str());
            params.insert("client_secret", self.client_secret.as_str());
            params.insert("redirect_uri", self.redirect_uri.as_str());
            params
        })
        .await
    }

    pub async fn refresh_tokens(&self, refresh_token: &str) -> PatreonResult<TokensResponse> {
        self.parse_token_request(&{
            let mut params = HashMap::new();
            params.insert("grant_type", "refresh_token");
            params.insert("client_id", self.client_id.as_str());
            params.insert("client_secret", self.client_secret.as_str());
            params.insert("refresh_token", refresh_token);
            params
        })
        .await
    }

    async fn parse_token_request(
        &self,
        params: &HashMap<&str, &str>,
    ) -> PatreonResult<TokensResponse> {
        let mut url = Url::parse(BASE_URI).unwrap();
        url.set_path("/api/oauth2/token");
        let response = self.agent.post(url).form(params).send().await?;
        let status = response.status();
        let text = response.text().await?;
        de_response(status, text)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TokensResponse {
    pub access_token: String,
    pub expires_in: u64,
    pub token_type: String,
    pub scope: String,
    pub refresh_token: String,
    #[serde(default)]
    pub version: String,
}

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    pub error: String,
}

fn de_response<T: for<'de> serde::Deserialize<'de>>(
    status: StatusCode,
    text: String,
) -> PatreonResult<T> {
    if status.is_success() {
        Ok(serde_json::from_str(text.as_str())?)
    } else {
        Err(PatreonError::PatreonOAuth(
            status,
            serde_json::from_str::<ErrorResponse>(text.as_str())?.error,
        ))
    }
}
