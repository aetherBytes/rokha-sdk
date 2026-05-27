use crate::api_client::{RokhaClient, SCHEMA_VERSION};
use crate::credentials;
use std::time::Duration;

const PROBE_TIMEOUT: Duration = Duration::from_millis(3000);

pub async fn version() {
    println!("ro {}", env!("CARGO_PKG_VERSION"));
    println!("schema {}", SCHEMA_VERSION);
}

/// `ro status` — thin-client check. Three lines:
///   identity  — logged in as <user> (tier) | not logged in
///   remote    — <base url> reachable | unreachable
///   schema    — matches CLI | drift | unreachable
pub async fn run(client: &RokhaClient) {
    println!("Rokha — {}", client.base_url());
    println!();

    // 1. Local identity
    print!("  identity  ");
    match credentials::load() {
        Some(c) => println!(
            "✓ {} ({}, tier: {})",
            c.identity.identity, c.identity.auth_method, c.identity.tier
        ),
        None => println!("✗ not logged in — run `ro login`"),
    }

    // 2. Remote reachability + schema (one HTTP call covers both via /api/schema/version)
    let http = reqwest::Client::builder()
        .timeout(PROBE_TIMEOUT)
        .build()
        .expect("failed to build http client");

    print!("  remote    ");
    let health_ok = match http
        .get(format!("{}/health", client.base_url()))
        .send()
        .await
    {
        Ok(res) if res.status().is_success() => {
            println!("✓ reachable");
            true
        }
        Ok(res) => {
            println!("✗ HTTP {}", res.status().as_u16());
            false
        }
        Err(e) => {
            println!("✗ unreachable ({})", e);
            false
        }
    };

    print!("  schema    ");
    if !health_ok {
        println!("— (remote down)");
        return;
    }
    match client.schema_version().await {
        Ok(sv) => {
            if sv.version == SCHEMA_VERSION {
                println!("✓ {} (matches CLI)", sv.version);
            } else {
                println!("⚠ server={} cli={}", sv.version, SCHEMA_VERSION);
                eprintln!();
                eprintln!("Schema drift. Update the CLI: brew upgrade rokha (or cargo install rokha-cli).");
            }
        }
        Err(e) => {
            println!("✗ /api/schema/version failed ({})", e);
        }
    }
}
