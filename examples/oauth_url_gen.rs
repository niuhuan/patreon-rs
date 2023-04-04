mod example_utils;

fn main() {
    let oauth = example_utils::oauth_client();
    println!("{}", oauth.get_authorization_url());
}
