use patreon::{OAuthClient, oauth::scopes};

fn env(name: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| panic!("{name} is required"))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let oauth = OAuthClient::new(env("CLIENT_ID"), env("CLIENT_SECRET"), env("REDIRECT_URL"));

    let auth_url = oauth.authorization_url(&[
        scopes::IDENTITY,
        scopes::IDENTITY_EMAIL,
        scopes::IDENTITY_MEMBERSHIPS,
    ]);
    println!("Authorization URL:\n{auth_url}\n");

    let auth_url_with_state = oauth.authorization_url_with_state(
        &[scopes::IDENTITY, scopes::IDENTITY_MEMBERSHIPS],
        "example_state_value",
    );
    println!("Authorization URL (with state):\n{auth_url_with_state}");

    Ok(())
}

