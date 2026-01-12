use patreon::{MemberResource, PatronStatus, PatreonUserClient, ResourceType};

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
    let campaign_id = env("CAMPAIGN_ID");

    // Requires scopes: `identity` + `identity.memberships`
    let resp = client.identity_with_memberships_and_campaign().await?;

    let mut is_active_patron = false;

    for item in resp.included {
        let Ok(m) = serde_json::from_value::<MemberResource>(item) else {
            continue;
        };
        if m.resource_type != ResourceType::Member {
            continue;
        }

        let relationships = m.relationships.unwrap_or_default();
        let Some(mid) = campaign_id_from_relationships(&relationships) else {
            continue;
        };
        if mid != campaign_id {
            continue;
        }

        let attrs = m.attributes.unwrap_or_default();
        if attrs.patron_status == PatronStatus::ActivePatron {
            is_active_patron = true;
            break;
        }
    }

    println!("campaign_id={campaign_id} is_active_patron={is_active_patron}");
    Ok(())
}
