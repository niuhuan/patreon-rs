use hmac::{Hmac, Mac};
use patreon::{WebhookValidator, webhook::WebhookEventType};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

fn compute_signature(secret: &str, body: &[u8]) -> String {
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC key");
    mac.update(body);
    let result = mac.finalize();
    hex::encode(result.into_bytes())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let secret = "example_webhook_secret";
    let validator = WebhookValidator::new(secret);

    let payload = r#"{
  "data": { "type": "member", "id": "123" },
  "links": { "self": "https://www.patreon.com/api/oauth2/v2/members/123" }
}"#;
    let payload_bytes = payload.as_bytes();

    let signature = compute_signature(secret, payload_bytes);
    assert!(validator.validate(payload_bytes, &signature));
    validator.validate_or_error(payload_bytes, &signature)?;

    let event = validator.parse_event(payload)?;
    println!("parsed_event.data: {}", event.data);

    let event2 = validator.validate_and_parse(payload_bytes, &signature)?;
    println!("validate_and_parse.data: {}", event2.data);

    let t = WebhookEventType::from_str("members:create");
    println!("WebhookEventType::from_str: {}", t.as_str());

    Ok(())
}

