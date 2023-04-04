mod example_utils;

#[tokio::main]
async fn main() {
    let oauth = example_utils::oauth_client();
    println!("{:?}", oauth.refresh_tokens(env!("REFRESH_TOKEN")).await);
}
