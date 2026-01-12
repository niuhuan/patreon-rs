use patreon::{MemberResource, PatreonUserClient, ResourceType};

fn env(name: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| panic!("{name} is required"))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = PatreonUserClient::new(env("USER_ACCESS_TOKEN"));

    // Requires scopes: `identity` + `identity.memberships`
    let resp = client.identity_with_memberships().await?;

    let mut count = 0usize;

    for item in resp.included {
        let Ok(resource) = serde_json::from_value::<MemberResource>(item) else {
            continue;
        };
        if resource.resource_type != ResourceType::Member {
            continue;
        }

        let attrs = resource.attributes.unwrap_or_default();
        println!(
            "member_id={} patron_status={:?} entitled_cents={} lifetime_cents={}",
            resource.id,
            attrs.patron_status,
            attrs.currently_entitled_amount_cents,
            attrs.lifetime_support_cents
        );
        count += 1;
    }

    println!("memberships_included={count}");
    Ok(())
}
