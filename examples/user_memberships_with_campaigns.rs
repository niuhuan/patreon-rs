use patreon::{CampaignResource, MemberResource, PatreonUserClient, ResourceType};
use std::collections::HashMap;

fn env(name: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| panic!("{name} is required"))
}

fn campaign_id_from_relationships(relationships: &serde_json::Value) -> Option<String> {
    relationships
        .get("campaign")?
        .get("data")?
        .get("id")?
        .as_str()
        .map(|s| s.to_string())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = PatreonUserClient::new(env("USER_ACCESS_TOKEN"));

    // Requires scopes: `identity` + `identity.memberships`
    let resp = client.identity_with_memberships_and_campaign().await?;

    let mut campaigns_by_id: HashMap<String, CampaignResource> = HashMap::new();
    let mut members: Vec<MemberResource> = Vec::new();

    for item in resp.included {
        if let Ok(c) = serde_json::from_value::<CampaignResource>(item.clone()) {
            if c.resource_type == ResourceType::Campaign {
                campaigns_by_id.insert(c.id.clone(), c);
                continue;
            }
        }

        if let Ok(m) = serde_json::from_value::<MemberResource>(item) {
            if m.resource_type == ResourceType::Member {
                members.push(m);
            }
        }
    }

    for m in members {
        let relationships = m.relationships.unwrap_or_default();
        let campaign_id = campaign_id_from_relationships(&relationships).unwrap_or_default();
        let campaign = campaigns_by_id.get(&campaign_id);

        let campaign_name = campaign
            .and_then(|c| c.attributes.as_ref())
            .map(|a| a.creation_name.as_str())
            .unwrap_or("");

        let attrs = m.attributes.unwrap_or_default();
        println!(
            "campaign_id={} campaign_name=\"{}\" patron_status={:?} entitled_cents={}",
            campaign_id, campaign_name, attrs.patron_status, attrs.currently_entitled_amount_cents
        );
    }

    Ok(())
}
