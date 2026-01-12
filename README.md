# Patreon API Rust Client

An async Rust client library for the Patreon API (v2).

## Contents

- Features
- Installation
- Clients
- Environment variables (for examples)
- Quick start
- Webhooks (validate + parse)
- Examples

## Features

- OAuth 2.0 authorization flow (authorization URL, code exchange, refresh token)
- User APIs (OAuth user access token)
- Creator/server APIs (creator access token)
- Webhook signature validation (HMAC-SHA256)
- Strongly-typed JSON:API models
- Async API based on `tokio`

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
patreon = "0.2"
tokio = { version = "1", features = ["full"] }
```

### TLS backend features

This crate uses `reqwest` internally. You can choose the TLS backend via crate features:

- `native-tls` (default): system TLS via `native-tls`
- `rustls`: `rustls` (reqwest `rustls` feature)

Examples:

```toml
# Default (native-tls)
patreon = "0.2"

# Force rustls
patreon = { version = "0.2", default-features = false, features = ["rustls"] }
```

## Clients

| Client | When to use | Token |
|---|---|---|
| `OAuthClient` | Build authorization URLs, exchange `code` for token, refresh token | `CLIENT_ID` + `CLIENT_SECRET` |
| `PatreonUserClient` | Third-party apps acting on behalf of a user | OAuth user access token |
| `PatreonCreatorClient` | Server-side access for a creator | Creator access token from Patreon developer portal |

## Environment variables (for examples)

The examples under `examples/` read credentials from environment variables:

- `CLIENT_ID`
- `CLIENT_SECRET`
- `REDIRECT_URL`
- `USER_ACCESS_TOKEN`
- `CREATOR_ACCESS_TOKEN`

Some examples also require additional variables (the program will tell you via `expect(...)`):

- `OAUTH_CODE`, `REFRESH_TOKEN`
- `PORT` (used by `oauth_localhost_login`, default: `8080`)
- `CAMPAIGN_ID`, `MEMBER_ID`, `POST_ID`
- `WEBHOOK_URI`

## Quick start

### OAuth (authorization URL, exchange code, refresh token)

```rust
use patreon::{OAuthClient, oauth::scopes};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let oauth = OAuthClient::new(
        "your_client_id",
        "your_client_secret",
        "https://your-app.com/oauth/callback",
    );

    let auth_url = oauth.authorization_url(&[
        scopes::IDENTITY,
        scopes::IDENTITY_EMAIL,
        scopes::IDENTITY_MEMBERSHIPS,
    ]);
    println!("Open this URL to authorize: {}", auth_url);

    let token = oauth.exchange_code("authorization_code_from_callback").await?;
    println!("Access Token: {}", token.access_token);
    println!("Expires At: {}", token.expires_at);

    if token.is_expiring_soon() {
        let new_token = oauth.refresh_token(&token.refresh_token).await?;
        println!("New Access Token: {}", new_token.access_token);
    }

    Ok(())
}
```

### User API (identity)

```rust
use patreon::PatreonUserClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = PatreonUserClient::new("user_access_token");
    let identity = client.identity().await?;
    println!("User ID: {}", identity.data.id);
    Ok(())
}
```

### Creator API (campaigns, members, posts, webhooks)

```rust
use patreon::PatreonCreatorClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = PatreonCreatorClient::new("creator_access_token");
    let campaigns = client.campaigns().await?;
    println!("Campaigns: {}", campaigns.data.len());
    Ok(())
}
```

## Webhooks (validate + parse)

Patreon sends webhook requests with:

- `X-Patreon-Signature`: HMAC-SHA256 hex signature of the raw request body
- `X-Patreon-Event`: event type string (e.g. `members:create`, `posts:publish`)

This crate provides `WebhookValidator` for signature verification and convenient parsing:

```rust
use patreon::{WebhookValidator, webhook::WebhookEventType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let secret = "your_webhook_secret";
    let validator = WebhookValidator::new(secret);

    // The raw request body bytes you received from your web framework:
    let body_bytes: &[u8] = br#"{"data":{"type":"member","id":"123"}}"#;
    let signature_header = "hex_signature_from_X_Patreon_Signature";

    // 1) Validate signature only
    if !validator.validate(body_bytes, signature_header) {
        return Err("invalid signature".into());
    }

    // 2) Validate + parse (bytes)
    let event = validator.validate_and_parse(body_bytes, signature_header)?;
    println!("event.data = {}", event.data);

    // 3) Parse only (string/bytes)
    let event2 = validator.parse_event_from_bytes(body_bytes)?;
    println!("event2.data = {}", event2.data);

    // Event type is carried by X-Patreon-Event:
    let event_type = WebhookEventType::from_str("members:create");
    println!("event_type = {}", event_type.as_str());

    Ok(())
}
```

## Examples

Run any example with:

`cargo run --example <name>`

Available examples:

- `oauth_authorization_url`
- `oauth_exchange_code`
- `oauth_localhost_login`
- `oauth_refresh_token`
- `user_identity`
- `user_memberships_list`
- `user_memberships_with_campaigns`
- `user_is_patron_of_campaign`
- `creator_campaigns`
- `creator_members`
- `creator_posts`
- `creator_webhooks`
- `creator_delete_webhook_cli`
- `webhook_validate`
