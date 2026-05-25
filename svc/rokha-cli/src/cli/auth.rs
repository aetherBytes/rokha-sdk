use crate::credentials::{self, Credentials, Identity};
use serde::{Deserialize, Serialize};
use std::process::{Command, Stdio};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const CLIENT_ID: &str = concat!("ro/", env!("CARGO_PKG_VERSION"));

#[derive(Deserialize, Debug)]
struct StartResponse {
    device_code: String,
    user_code: String,
    verification_uri: String,
    verification_uri_complete: String,
    expires_in: i64,
    interval: i64,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "status", rename_all = "snake_case")]
enum PollResponse {
    Pending,
    SlowDown {
        interval: i64,
    },
    Authorized {
        jwt: String,
        identity: IdentityResponse,
    },
    Expired,
    Denied,
}

#[derive(Deserialize, Debug)]
struct IdentityResponse {
    user_id: String,
    identity: String,
    auth_method: String,
    tier: String,
}

#[derive(Serialize)]
struct StartBody {
    scope: &'static str,
    client: &'static str,
}

/// `ro login` — drives the device flow end-to-end.
pub async fn login(base_url: &str) -> i32 {
    let http = reqwest::Client::new();

    let start: StartResponse = match http
        .post(format!("{}/api/auth/cli/start", base_url))
        .json(&StartBody { scope: "cli", client: CLIENT_ID })
        .send()
        .await
        .and_then(|r| r.error_for_status())
    {
        Ok(resp) => match resp.json().await {
            Ok(s) => s,
            Err(e) => {
                eprintln!("ro login: malformed /api/auth/cli/start response: {}", e);
                return 1;
            }
        },
        Err(e) => {
            eprintln!("ro login: could not reach {} ({})", base_url, e);
            eprintln!("Set ROKHA_BASE_URL or check connectivity. Then re-run `ro login`.");
            return 1;
        }
    };

    println!();
    println!("Open the link below in your browser, log in, and enter the code:");
    println!();
    println!("    {}", start.verification_uri);
    println!("    code: \x1b[1m{}\x1b[0m", start.user_code);
    println!();
    println!("Or open this link directly (skips the typing step):");
    println!("    {}", start.verification_uri_complete);
    println!();

    if open_browser(&start.verification_uri_complete) {
        println!("(opened in your browser)");
    } else {
        println!("(could not auto-open browser — visit the link manually)");
    }
    println!();
    println!("Waiting for authorization (expires in {} seconds)…", start.expires_in);

    let started = std::time::Instant::now();
    let max_wait = Duration::from_secs(start.expires_in as u64);
    let mut interval = Duration::from_secs(start.interval.max(1) as u64);

    loop {
        if started.elapsed() > max_wait {
            eprintln!();
            eprintln!("ro login: timed out before the code was authorized.");
            return 1;
        }
        tokio::time::sleep(interval).await;

        let poll_res = http
            .post(format!("{}/api/auth/cli/poll", base_url))
            .json(&serde_json::json!({ "device_code": start.device_code }))
            .send()
            .await;

        let resp = match poll_res {
            Ok(r) => r,
            Err(e) => {
                eprintln!("ro login: poll failed: {}. Retrying…", e);
                continue;
            }
        };

        if !resp.status().is_success() {
            eprintln!("ro login: server returned {}; aborting.", resp.status());
            return 1;
        }

        let body: PollResponse = match resp.json().await {
            Ok(b) => b,
            Err(e) => {
                eprintln!("ro login: malformed poll response: {}", e);
                continue;
            }
        };

        match body {
            PollResponse::Pending => {
                print!(".");
                let _ = std::io::Write::flush(&mut std::io::stdout());
            }
            PollResponse::SlowDown { interval: i } => {
                interval = Duration::from_secs(i.max(1) as u64);
                println!();
                println!("(server asked us to slow down — new interval {}s)", i);
            }
            PollResponse::Expired => {
                eprintln!();
                eprintln!("ro login: code expired. Run `ro login` again.");
                return 1;
            }
            PollResponse::Denied => {
                eprintln!();
                eprintln!("ro login: authorization was denied.");
                return 1;
            }
            PollResponse::Authorized { jwt, identity } => {
                let creds = Credentials {
                    jwt,
                    identity: Identity {
                        user_id: identity.user_id,
                        identity: identity.identity,
                        auth_method: identity.auth_method,
                        tier: identity.tier,
                    },
                    saved_at: SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .map(|d| d.as_secs() as i64)
                        .unwrap_or(0),
                    base_url: base_url.to_string(),
                };
                if let Err(e) = credentials::save(&creds) {
                    eprintln!();
                    eprintln!("ro login: could not write credentials: {}", e);
                    return 1;
                }
                println!();
                println!("✓ Logged in as \x1b[1m{}\x1b[0m ({}, tier: {})",
                    creds.identity.identity, creds.identity.auth_method, creds.identity.tier);
                println!("  credentials: {}", credentials::credentials_path().display());
                return 0;
            }
        }
    }
}

/// `ro whoami` — read credentials, decode JWT, print identity.
pub async fn whoami() -> i32 {
    let creds = match credentials::load() {
        Some(c) => c,
        None => {
            eprintln!("Not logged in. Run `ro login`.");
            return 1;
        }
    };

    let claims = credentials::decode_payload(&creds.jwt);
    let exp_str = claims
        .as_ref()
        .map(|c| {
            chrono_like_format(c.exp).unwrap_or_else(|| format!("(exp={})", c.exp))
        })
        .unwrap_or_else(|| "(unreadable JWT payload)".to_string());

    println!("Logged in as: {}", creds.identity.identity);
    println!("  user_id:     {}", creds.identity.user_id);
    println!("  auth method: {}", creds.identity.auth_method);
    println!("  tier:        {}", creds.identity.tier);
    println!("  base url:    {}", creds.base_url);
    println!("  jwt expires: {}", exp_str);
    if let Some(c) = &claims {
        if !c.scopes.is_empty() {
            println!("  scopes:      {}", c.scopes.join(", "));
        }
    }
    println!("  file:        {}", credentials::credentials_path().display());
    0
}

/// `ro logout` — delete credentials.
pub async fn logout() -> i32 {
    match credentials::delete() {
        Ok(true) => {
            println!("Logged out.");
            0
        }
        Ok(false) => {
            println!("Already logged out (no credentials file).");
            0
        }
        Err(e) => {
            eprintln!("ro logout: could not remove credentials: {}", e);
            1
        }
    }
}

fn open_browser(url: &str) -> bool {
    let (cmd, args): (&str, Vec<&str>) = if cfg!(target_os = "macos") {
        ("open", vec![url])
    } else if cfg!(target_os = "windows") {
        ("cmd", vec!["/C", "start", "", url])
    } else {
        ("xdg-open", vec![url])
    };
    Command::new(cmd)
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map(|mut c| {
            // Don't block; we don't care about the browser's exit status.
            let _ = c.wait();
            true
        })
        .unwrap_or(false)
}

/// Minimal Unix-epoch → human formatter so we don't pull in chrono.
fn chrono_like_format(epoch_secs: i64) -> Option<String> {
    if epoch_secs <= 0 {
        return None;
    }
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()?
        .as_secs() as i64;
    let delta = epoch_secs - now;
    let suffix = if delta < 0 {
        format!("expired {} ago", human_duration((-delta) as u64))
    } else {
        format!("in {}", human_duration(delta as u64))
    };
    // Best-effort UTC date via `date -u -r` if available; fall back to epoch.
    let date_str = Command::new("date")
        .args(["-u", "-r", &epoch_secs.to_string(), "+%Y-%m-%d %H:%M UTC"])
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                Some(String::from_utf8_lossy(&o.stdout).trim().to_string())
            } else {
                None
            }
        })
        .unwrap_or_else(|| format!("epoch {}", epoch_secs));
    Some(format!("{} ({})", date_str, suffix))
}

fn human_duration(secs: u64) -> String {
    let days = secs / 86_400;
    let hours = (secs % 86_400) / 3_600;
    if days > 0 {
        format!("{}d {}h", days, hours)
    } else if hours > 0 {
        format!("{}h", hours)
    } else {
        format!("{}m", secs / 60)
    }
}
