PATREON
=======

A patreon client crate for rust.

```rust
async fn example() {
    // OAuth client
    let client = PatreonOAuth {
        client_id: env!("CLIENT_ID").to_string(),
        client_secret: env!("CLIENT_SECRET").to_string(),
        redirect_uri: env!("REDIRECT_URI").to_string(),
        ..Default::default()
    };
    // authorization by url
    println!("{}", oauth.get_authorization_url());
    oauth.get_tokens("");

    // Api Clinet
    let api = PatreonApi {
        access_token: env!("ACCESS_TOKEN").to_string(),
        ..Default::default()
    };
    println!("{:?}", api.ident().await);
  
    // webhook
    let webhook = Webhook {
        webhook_secret,
    };
    webhook.check_signature(...);
    webhook.parse_event(...);
}
```

## Features

- [x] OAuth
  - [x] Get authorization url
  - [x] Get tokens from code
  - [x] Refresh tokens
- [x] Api
  - [x] Current user 
  - [x] Identity
  - [x] Identity include Memberships
  - [x] Identity include Campaign
- [x] Webhook
  - [x] Check check_signature
  - [x] Parse