use crate::api_client::{RokhaClient, SCHEMA_VERSION};
use std::time::Duration;

const PROBE_TIMEOUT: Duration = Duration::from_millis(1500);

struct ServiceProbe {
    name: &'static str,
    url: &'static str,
}

const SERVICES: &[ServiceProbe] = &[
    ServiceProbe { name: "erebus         ", url: "http://localhost:3000/health" },
    ServiceProbe { name: "rokha-agents   ", url: "http://localhost:9003/health" },
    ServiceProbe { name: "rokha-harnesses", url: "http://localhost:9004/health" },
    ServiceProbe { name: "rokha-protocols", url: "http://localhost:8001/health" },
];

pub async fn version() {
    println!("ro {}", env!("CARGO_PKG_VERSION"));
    println!("schema {}", SCHEMA_VERSION);
}

async fn probe(client: &reqwest::Client, url: &str) -> Result<serde_json::Value, String> {
    let res = client
        .get(url)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !res.status().is_success() {
        return Err(format!("HTTP {}", res.status().as_u16()));
    }
    res.json::<serde_json::Value>().await.map_err(|e| e.to_string())
}

fn extract_status(body: &serde_json::Value) -> &str {
    body.get("status").and_then(|v| v.as_str()).unwrap_or("ok")
}

/// Agents-service health embeds the LLM factory's mode at
/// `components.llm_service.mode`: `"proxy" | "direct" | "none"`.
fn extract_llm_mode(body: &serde_json::Value) -> Option<&str> {
    body.get("components")?
        .get("llm_service")?
        .get("mode")?
        .as_str()
}

fn looks_like_no_llm(body: &serde_json::Value) -> bool {
    if extract_llm_mode(body) == Some("none") {
        return true;
    }
    // Fallback: pre-slice-2 agents reported "No working models available"
    // without an explicit mode field. Keep the substring check until all
    // deployed agent images carry the new field.
    let s = serde_json::to_string(body).unwrap_or_default().to_lowercase();
    s.contains("no working llm") || s.contains("no models available")
}

pub async fn run(client: &RokhaClient) {
    println!("Rokha — {}", client.base_url());
    println!();

    let http = reqwest::Client::builder()
        .timeout(PROBE_TIMEOUT)
        .build()
        .expect("failed to build http client");

    let mut any_up = false;
    let mut agents_needs_llm = false;

    for svc in SERVICES {
        print!("  {}  ", svc.name);
        match probe(&http, svc.url).await {
            Ok(body) => {
                let status = extract_status(&body);
                if svc.name.contains("rokha-agents") {
                    if looks_like_no_llm(&body) {
                        println!("⚠ {} — awaiting credentials (no LLM)", status);
                        agents_needs_llm = true;
                    } else {
                        match extract_llm_mode(&body) {
                            Some("proxy") => println!("✓ {} (LLM: proxy via rokha account)", status),
                            Some("direct") => println!("✓ {} (LLM: direct anthropic)", status),
                            _ => println!("✓ {}", status),
                        }
                    }
                } else {
                    println!("✓ {}", status);
                }
                any_up = true;
            }
            Err(e) => {
                println!("✗ unreachable ({})", e);
            }
        }
    }

    println!();
    print!("  schema           ");
    if !any_up {
        println!("— (services down)");
        println!();
        eprintln!("Nothing is responding. Run `ro up` to start the local stack.");
        return;
    }

    match client.schema_version().await {
        Ok(sv) => {
            if sv.version == SCHEMA_VERSION {
                println!("✓ {} (matches CLI)", sv.version);
            } else {
                println!("⚠ server={} cli={}", sv.version, SCHEMA_VERSION);
                eprintln!();
                eprintln!("Schema drift. Update the CLI: cargo install rokha-cli");
            }
        }
        Err(e) => {
            println!("✗ /api/schema/version failed ({})", e);
        }
    }

    if agents_needs_llm {
        println!();
        eprintln!("rokha-agents is up but has no LLM. Either set");
        eprintln!("ANTHROPIC_KEY_ROKHA_AGENT in ~/.rokha/env and re-run `ro up`,");
        eprintln!("or wait for v0.4 (routes LLM calls through your Rokha account).");
    }
}
