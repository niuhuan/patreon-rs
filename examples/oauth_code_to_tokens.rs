mod example_utils;

#[tokio::main]
async fn main() {
    let oauth = example_utils::oauth_client();
    println!("{:?}", oauth.get_tokens(env!("CODE")).await);
}
