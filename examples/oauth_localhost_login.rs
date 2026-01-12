use patreon::{OAuthClient, oauth::scopes};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use tokio::sync::oneshot;

fn env(name: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| panic!("{name} is required"))
}

fn env_port() -> u16 {
    match std::env::var("PORT") {
        Ok(v) => v
            .parse::<u16>()
            .unwrap_or_else(|_| panic!("PORT must be a valid u16, got: {v}")),
        Err(_) => 8080,
    }
}

fn open_browser(url: &str) {
    if let Ok(browser) = std::env::var("BROWSER") {
        let _ = std::process::Command::new(browser).arg(url).spawn();
        return;
    }

    if cfg!(target_os = "macos") {
        let _ = std::process::Command::new("open").arg(url).spawn();
    } else if cfg!(target_os = "windows") {
        let _ = std::process::Command::new("cmd")
            .args(["/C", "start", url])
            .spawn();
    } else {
        let _ = std::process::Command::new("xdg-open").arg(url).spawn();
    }
}

fn http_ok(stream: &mut TcpStream, body: &str) -> std::io::Result<()> {
    let body_bytes = body.as_bytes();
    write!(
        stream,
        "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body_bytes.len()
    )?;
    stream.write_all(body_bytes)?;
    Ok(())
}

fn http_redirect(stream: &mut TcpStream, location: &str) -> std::io::Result<()> {
    write!(
        stream,
        "HTTP/1.1 302 Found\r\nLocation: {}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        location
    )?;
    Ok(())
}

fn read_http_request_path(stream: &mut TcpStream) -> std::io::Result<String> {
    let mut buf = [0u8; 8192];
    let n = stream.read(&mut buf)?;
    let req = String::from_utf8_lossy(&buf[..n]);
    let first_line = req.lines().next().unwrap_or_default();
    let mut parts = first_line.split_whitespace();
    let _method = parts.next().unwrap_or_default();
    let path = parts.next().unwrap_or("/");
    Ok(path.to_string())
}

fn parse_query(path: &str) -> (String, Vec<(String, String)>) {
    let mut parts = path.splitn(2, '?');
    let route = parts.next().unwrap_or("/").to_string();
    let query = parts.next().unwrap_or("");
    let mut out = Vec::new();
    for pair in query.split('&').filter(|s| !s.is_empty()) {
        let mut kv = pair.splitn(2, '=');
        let k = kv.next().unwrap_or("").to_string();
        let v = kv.next().unwrap_or("");
        let v = urlencoding::decode(v).unwrap_or_else(|_| v.into()).to_string();
        out.push((k, v));
    }
    (route, out)
}

fn gen_state() -> String {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    format!("state-{nanos}")
}

fn run_local_server(
    listener: TcpListener,
    auth_url: String,
    expected_state: String,
    tx: oneshot::Sender<String>,
) -> std::io::Result<()> {
    let port = listener.local_addr()?.port();
    eprintln!("Listening on http://localhost:{port}");

    for stream in listener.incoming() {
        let mut stream = match stream {
            Ok(s) => s,
            Err(e) => {
                eprintln!("accept failed: {e}");
                continue;
            }
        };

        let path = match read_http_request_path(&mut stream) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("read request failed: {e}");
                continue;
            }
        };

        let (route, query) = parse_query(&path);

        if route == "/favicon.ico" {
            let _ = http_ok(&mut stream, "");
            continue;
        }

        if route == "/" {
            let _ = http_redirect(&mut stream, &auth_url);
            continue;
        }

        if route != "/callback" {
            let _ = http_ok(
                &mut stream,
                "<h1>Not Found</h1><p>Try <a href=\"/\">/</a>.</p>",
            );
            continue;
        }

        let mut code: Option<String> = None;
        let mut state: Option<String> = None;
        for (k, v) in query {
            match k.as_str() {
                "code" => code = Some(v),
                "state" => state = Some(v),
                _ => {}
            }
        }

        if state.as_deref() != Some(&expected_state) {
            let _ = http_ok(
                &mut stream,
                "<h1>Invalid state</h1><p>Please restart the flow.</p>",
            );
            continue;
        }

        let Some(code) = code else {
            let _ = http_ok(
                &mut stream,
                "<h1>Missing code</h1><p>No authorization code found in callback.</p>",
            );
            continue;
        };

        let _ = http_ok(
            &mut stream,
            "<h1>Success</h1><p>You can close this tab and return to the terminal.</p>",
        );

        let _ = tx.send(code);
        break;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client_id = env("CLIENT_ID");
    let client_secret = env("CLIENT_SECRET");

    let port = env_port();
    let listener = TcpListener::bind(("127.0.0.1", port))?;
    let port = listener.local_addr()?.port();
    let redirect_url = format!("http://localhost:{port}/callback");

    let oauth = OAuthClient::new(client_id, client_secret, redirect_url);
    let state = gen_state();

    let auth_url = oauth.authorization_url_with_state(
        &[
            scopes::IDENTITY,
            scopes::IDENTITY_EMAIL,
            scopes::IDENTITY_MEMBERSHIPS,
        ],
        &state,
    );

    let (tx, rx) = oneshot::channel::<String>();
    std::thread::spawn(move || {
        if let Err(e) = run_local_server(listener, auth_url, state, tx) {
            eprintln!("local server error: {e}");
        }
    });

    let local_entry = format!("http://localhost:{port}/");
    eprintln!("Opening browser to start login:\n{local_entry}\n");
    open_browser(&local_entry);

    let code = rx.await?;
    let token = oauth.exchange_code(&code).await?;

    println!("access_token: {}", token.access_token);
    println!("refresh_token: {}", token.refresh_token);
    println!("expires_at: {}", token.expires_at);
    println!("token_type: {}", token.token_type);
    println!("scope: {}", token.scope);

    Ok(())
}
