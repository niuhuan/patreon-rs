use patreon::PatreonCreatorClient;

fn env(name: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| panic!("{name} is required"))
}

fn usage() -> ! {
    eprintln!("Usage: cargo run --example creator_delete_webhook_cli -- <webhook_id>");
    std::process::exit(2);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let webhook_id = std::env::args().nth(1).unwrap_or_else(|| usage());

    let client = PatreonCreatorClient::new(env("CREATOR_ACCESS_TOKEN"));
    client.delete_webhook(&webhook_id).await?;

    println!("deleted_webhook.id: {webhook_id}");
    Ok(())
}

