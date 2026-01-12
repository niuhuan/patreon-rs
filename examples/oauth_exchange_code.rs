use patreon::OAuthClient;

fn env(name: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| panic!("{name} is required"))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let oauth = OAuthClient::new(env("CLIENT_ID"), env("CLIENT_SECRET"), env("REDIRECT_URL"));
    let code = env("OAUTH_CODE");

    let token = oauth.exchange_code(&code).await?;
    println!("access_token: {}", token.access_token);
    println!("refresh_token: {}", token.refresh_token);
    println!("expires_at: {}", token.expires_at);
    println!("token_type: {}", token.token_type);
    println!("scope: {}", token.scope);
    println!("is_expired: {}", token.is_expired());
    println!("is_expiring_soon: {}", token.is_expiring_soon());

    Ok(())
}

