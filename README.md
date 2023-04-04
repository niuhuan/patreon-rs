PATREON
=======

A patreon client

```rust
async fn example() {
    // OAuth client
    let client = PatreonOAuth {
        client_id: env!("CLIENT_ID").to_string(),
        client_secret: env!("CLIENT_SECRET").to_string(),
        redirect_uri: env!("REDIRECT_URI").to_string(),
        ..Default::default()
    };
    // get authorization url
    println!("{}", oauth.get_authorization_url());
    // ...
}
```

## Features

- [x] OAuth
  - [x] Get authorization url
  - [x] Get tokens from code
  - [x] Refresh tokens