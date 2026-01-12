use patreon::{
    PatreonCreatorClient,
    creator_client::{CreateWebhookRequest, webhook_triggers},
};

fn env(name: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| panic!("{name} is required"))
}

fn opt_env(name: &str) -> Option<String> {
    std::env::var(name).ok().filter(|v| !v.trim().is_empty())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = PatreonCreatorClient::new(env("CREATOR_ACCESS_TOKEN"));

    let webhooks = client.webhooks().await?;
    println!("webhooks.count: {}", webhooks.data.len());

    let webhook_uri = match opt_env("WEBHOOK_URI") {
        Some(v) => v,
        None => {
            println!("WEBHOOK_URI not set; skipping create/update/delete examples.");
            return Ok(());
        }
    };

    let campaigns = client.campaigns().await?;
    let campaign_id = opt_env("CAMPAIGN_ID")
        .or_else(|| campaigns.data.first().map(|c| c.id.clone()))
        .expect("CAMPAIGN_ID is required (or ensure the token has at least one campaign)");

    let created = client
        .create_webhook(&CreateWebhookRequest {
            uri: webhook_uri,
            campaign_id,
            triggers: vec![
                webhook_triggers::MEMBERS_CREATE.to_string(),
                webhook_triggers::POSTS_PUBLISH.to_string(),
            ],
        })
        .await?;

    let webhook_id = created.data.id.clone();
    println!("created_webhook.id: {webhook_id}");

    let updated = client
        .update_webhook(&webhook_id, None, Some(&[webhook_triggers::MEMBERS_CREATE]), Some(true))
        .await?;
    println!("updated_webhook.id: {}", updated.data.id);

    client.delete_webhook(&webhook_id).await?;
    println!("deleted_webhook.id: {webhook_id}");

    Ok(())
}
