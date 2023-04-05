mod api_utils;

#[tokio::main]
async fn main() {
    let api = api_utils::api_client();
    println!("{:?}", api.current_user().await);
}
