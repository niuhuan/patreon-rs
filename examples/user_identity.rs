use patreon::{PatreonUserClient, user_client::identity_fields};

fn env(name: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| panic!("{name} is required"))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = PatreonUserClient::new(env("USER_ACCESS_TOKEN"));

    if let Ok(base_url) = std::env::var("PATREON_BASE_URL") {
        client = client.with_base_url(base_url);
    }

    let http_client = reqwest::Client::builder()
        .user_agent("patreon-rust-example")
        .build()?;
    client = client.with_http_client(http_client);

    let identity = client.identity().await?;
    println!("identity.user_id: {}", identity.data.id);

    let identity_custom = client
        .identity_with_fields(&[
            identity_fields::EMAIL,
            identity_fields::FULL_NAME,
            identity_fields::IMAGE_URL,
        ])
        .await?;
    println!("identity_with_fields.user_id: {}", identity_custom.data.id);

    let identity_with_memberships = client.identity_with_memberships().await?;
    println!(
        "identity_with_memberships.user_id: {}",
        identity_with_memberships.data.id
    );

    let identity_with_memberships_and_campaign =
        client.identity_with_memberships_and_campaign().await?;
    println!(
        "identity_with_memberships_and_campaign.user_id: {}",
        identity_with_memberships_and_campaign.data.id
    );

    let identity_full = client.identity_full().await?;
    println!("identity_full.user_id: {}", identity_full.data.id);

    Ok(())
}

