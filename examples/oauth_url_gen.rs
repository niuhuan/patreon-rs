mod oauth_utils;

fn main() {
    let oauth = oauth_utils::oauth_client();
    println!("{}", oauth.get_authorization_url());
}
