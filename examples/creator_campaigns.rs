use patreon::PatreonCreatorClient;

fn env(name: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| panic!("{name} is required"))
}

fn opt_env(name: &str) -> Option<String> {
    std::env::var(name).ok().filter(|v| !v.trim().is_empty())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = PatreonCreatorClient::new(env("CREATOR_ACCESS_TOKEN"));

    if let Ok(base_url) = std::env::var("PATREON_BASE_URL") {
        client = client.with_base_url(base_url);
    }

    let http_client = reqwest::Client::builder()
        .user_agent("patreon-rust-example")
        .build()?;
    client = client.with_http_client(http_client);

    let campaigns = client.campaigns().await?;
    println!("campaigns.count: {}", campaigns.data.len());

    let campaigns_with_details = client.campaigns_with_details().await?;
    println!(
        "campaigns_with_details.count: {}",
        campaigns_with_details.data.len()
    );

    let campaign_id = opt_env("CAMPAIGN_ID")
        .or_else(|| campaigns.data.first().map(|c| c.id.clone()))
        .expect("CAMPAIGN_ID is required (or ensure the token has at least one campaign)");

    let campaign = client.campaign(&campaign_id).await?;
    println!("campaign.id: {}", campaign.data.id);

    let campaign_with_tiers_and_benefits = client.campaign_with_tiers_and_benefits(&campaign_id).await?;
    println!(
        "campaign_with_tiers_and_benefits.id: {}",
        campaign_with_tiers_and_benefits.data.id
    );

    Ok(())
}

