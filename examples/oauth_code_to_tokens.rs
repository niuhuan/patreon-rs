mod oauth_utils;

#[tokio::main]
async fn main() {
    let oauth = oauth_utils::oauth_client();
    println!("{:?}", oauth.get_tokens(env!("CODE")).await);
}
