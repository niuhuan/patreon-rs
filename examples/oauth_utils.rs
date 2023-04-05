use patreon::PatreonOAuth;

pub fn oauth_client() -> PatreonOAuth {
    PatreonOAuth {
        client_id: env!("CLIENT_ID").to_string(),
        client_secret: env!("CLIENT_SECRET").to_string(),
        redirect_uri: env!("REDIRECT_URI").to_string(),
        ..Default::default()
    }
}

#[allow(dead_code)]
fn main() {}
