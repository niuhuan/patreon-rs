use patreon::PatreonApi;

pub fn api_client() -> PatreonApi {
    PatreonApi {
        access_token: env!("ACCESS_TOKEN").to_string(),
        ..Default::default()
    }
}

#[allow(dead_code)]
fn main() {}
