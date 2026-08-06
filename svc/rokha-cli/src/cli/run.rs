//! `ro run <slug>` — run a registry skill FOR REAL in the platform sandbox.
//!
//! The local agent's "do it" verb, and the product thesis from the terminal: a
//! skill executes on a throwaway, isolated, egress-only cloud machine — nothing
//! installs locally, no secrets travel — and the trace is the receipt.
//!
//! Two doors, same contract as the browser (runner pays, always):
//! - logged in  → `POST /api/runtime/run` (your tier's daily run quota)
//! - anonymous  → `POST /api/runtime/taste` (the free public taste — a small
//!   per-session daily allowance, inside the global daily budget)
//!
//! Every limit denial is LOUD and names the ladder up. Run-Real-or-Raise: this
//! command never simulates — if the platform can't run it, you get the typed
//! refusal, not plausible fake output.

use crate::api_client::{Listing, RokhaClient};
use crate::credentials;
use crate::theme::Theme;
use serde_json::{json, Value};
use std::io::Write;

const POLL_MS: u64 = 2000;
const MAX_POLLS: u32 = 240; // 8 minutes — past every sandbox turn cap

pub async fn run(
    client: &RokhaClient,
    slug: &str,
    provider: Option<&str>,
    instruction: Option<&str>,
    params: &[String],
) -> i32 {
    let t = Theme::detect();
    let http = crate::api_client::http_client();

    // ── 1. Resolve the listing ────────────────────────────────────────────
    let listing = match resolve(client, slug, provider).await {
        Ok(l) => l,
        Err(code) => return code,
    };
    let skill_provider = listing.provider().to_string();
    let skill_slug = match &listing.external_id {
        Some(id) if !id.is_empty() => id.clone(),
        _ => {
            eprintln!(
                "'{slug}' resolved to a listing with no runnable id — try `ro tools info {slug}`."
            );
            return 1;
        }
    };

    let params_json = match parse_params(params) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("bad --param: {e} (expected key=value)");
            return 1;
        }
    };

    // ── 2. Say what's about to happen, honestly ──────────────────────────
    println!(
        "{} {} {}",
        t.ice_bold(listing.name()),
        t.dim(&format!("[{}]", skill_provider)),
        if listing.probe_verified.unwrap_or(false) {
            t.ok("✓ verified")
        } else {
            String::new()
        }
    );
    let creds = credentials::load();
    let (door_line, spend_line) = match &creds {
        Some(c) => (
            "your account's run quota".to_string(),
            format!("runs as {} ({})", c.identity.identity, c.identity.tier),
        ),
        None => (
            "one of the day's free public taste runs".to_string(),
            "anonymous — log in to run on your own quota".to_string(),
        ),
    };
    println!(
        "{}",
        t.dim(&format!(
            "Runs on a throwaway isolated cloud sandbox · spends {door_line} · {spend_line}"
        ))
    );

    // ── 3. Fire the run ──────────────────────────────────────────────────
    // The trace anchor must be a REAL harness row (the trace write is
    // FK-checked) — a made-up UUID would run fine but lose the receipt.
    let anon_for_anchor = if creds.is_none() {
        Some(anon_session_id())
    } else {
        None
    };
    let harness_id = match create_anchor(
        &http,
        client.base_url(),
        creds.as_ref().map(|c| c.jwt.as_str()),
        anon_for_anchor.as_deref(),
        listing.name(),
        &skill_provider,
        instruction,
        &params_json,
    )
    .await
    {
        Ok(id) => id,
        Err(e) => {
            eprintln!("could not create the run's trace anchor: {e}");
            return 1;
        }
    };
    let (ack, anon_session) = match &creds {
        Some(c) => {
            let body = json!({
                "skill_provider": skill_provider,
                "skill_slug": skill_slug,
                "instruction": instruction.unwrap_or(""),
                "params": params_json,
                "harness_id": harness_id,
            });
            let resp = http
                .post(format!("{}/api/runtime/run", client.base_url()))
                .bearer_auth(&c.jwt)
                .json(&body)
                .send()
                .await;
            (resp, None)
        }
        None => {
            let session = anon_for_anchor.clone().unwrap_or_else(anon_session_id);
            let body = json!({
                "anon_session_id": session,
                "skill_provider": skill_provider,
                "skill_slug": skill_slug,
                "instruction": instruction.unwrap_or(""),
                "params": params_json,
                "harness_id": harness_id,
            });
            let resp = http
                .post(format!("{}/api/runtime/taste", client.base_url()))
                .json(&body)
                .send()
                .await;
            (resp, Some(session))
        }
    };

    let run_id = match read_ack(ack, t).await {
        Ok(id) => id,
        Err(code) => return code,
    };
    println!("{}", t.dim(&format!("run {run_id} dispatched")));
    println!();

    // ── 4. Stream progress until the run settles ─────────────────────────
    let final_status = poll_progress(&http, client.base_url(), &run_id, t).await;

    // ── 5. The verdict + the receipt ─────────────────────────────────────
    match final_status.as_deref() {
        Some("success") => println!("\n{} run complete", t.ok("✓")),
        Some(other) => {
            println!("\n{} run ended: {other}", t.amber("⚠"));
        }
        None => println!(
            "\n{} still running server-side — it outlived this poll window, not the sandbox.",
            t.amber("⚠")
        ),
    }

    // Anon traces are readable back through the anon rail — print the receipt.
    if let Some(session) = anon_session {
        print_anon_trace(&http, client.base_url(), &session, &harness_id, t).await;
    }
    println!(
        "{}",
        t.faint("Full trace: rokha.ai → TRACES (the run is its own receipt)")
    );
    if final_status.as_deref() == Some("success") {
        0
    } else {
        1
    }
}

/// Exact-match the slug against name/external_id; on ambiguity prefer verified,
/// then the official provider. No match → show the near misses and refuse.
async fn resolve(client: &RokhaClient, slug: &str, provider: Option<&str>) -> Result<Listing, i32> {
    let page = match client.search_registry(slug, 50).await {
        Ok(p) => p,
        Err(e) => {
            eprintln!("registry search failed: {e}");
            return Err(1);
        }
    };
    let lower = slug.to_lowercase();
    let mut exact: Vec<Listing> = page
        .items
        .iter()
        .filter(|l| {
            (l.name().to_lowercase() == lower
                || l.external_id.as_deref().unwrap_or("").to_lowercase() == lower)
                && provider.is_none_or(|p| l.provider().eq_ignore_ascii_case(p))
        })
        .cloned()
        .collect();
    exact.sort_by_key(|l| (!l.probe_verified.unwrap_or(false), l.provider() != "rokha"));
    if let Some(l) = exact.into_iter().next() {
        return Ok(l);
    }
    if page.items.is_empty() {
        eprintln!("No listing matches '{slug}'.");
    } else {
        eprintln!("No exact match for '{slug}'. Close:");
        for l in page.items.iter().take(5) {
            eprintln!(
                "  {}  [{}]  {}",
                l.name(),
                l.provider(),
                l.external_id.as_deref().unwrap_or("")
            );
        }
        eprintln!("Run one with: ro run <name> --provider <provider>");
    }
    Err(1)
}

/// Create the run's anchor harness through whichever rail matches the caller:
/// authed → `POST /api/harnesses` (owner from the JWT), anon → the
/// `/api/anon/harnesses` proxy (owner from the session header). Either way the
/// server stamps the owner — the body's wallet field is overridden.
#[allow(clippy::too_many_arguments)]
async fn create_anchor(
    http: &reqwest::Client,
    base: &str,
    jwt: Option<&str>,
    anon_session: Option<&str>,
    skill_name: &str,
    provider: &str,
    instruction: Option<&str>,
    params: &Value,
) -> Result<String, String> {
    let body = json!({
        "wallet_address": "",
        "harness_type": "skill",
        "key": format!("ro.run.{}", skill_name.to_lowercase().replace(' ', "-")),
        "content": {
            "skill": skill_name,
            "provider": provider,
            "instruction": instruction.unwrap_or(""),
            "params": params,
        },
        "summary": format!("ro run {skill_name}"),
        "created_by": "ro",
    });
    let req = match (jwt, anon_session) {
        (Some(token), _) => http
            .post(format!("{base}/api/harnesses"))
            .bearer_auth(token),
        (None, Some(session)) => http
            .post(format!("{base}/api/anon/harnesses"))
            .header("x-anon-session-id", session),
        (None, None) => return Err("no identity for the anchor".into()),
    };
    let resp = req.json(&body).send().await.map_err(|e| e.to_string())?;
    let status = resp.status();
    let v: Value = resp.json().await.map_err(|e| e.to_string())?;
    if !status.is_success() {
        return Err(v
            .get("message")
            .and_then(|m| m.as_str())
            .unwrap_or("harness create refused")
            .to_string());
    }
    v.get("data")
        .and_then(|d| d.get("id"))
        .and_then(|i| i.as_str())
        .map(String::from)
        .ok_or_else(|| format!("no harness id in response: {v}"))
}

fn parse_params(params: &[String]) -> Result<Value, String> {
    let mut map = serde_json::Map::new();
    for p in params {
        let (k, v) = p.split_once('=').ok_or_else(|| p.clone())?;
        if k.is_empty() {
            return Err(p.clone());
        }
        map.insert(k.to_string(), Value::String(v.to_string()));
    }
    Ok(Value::Object(map))
}

/// The anon free-run scope, persisted so retries and tomorrow's run share one
/// identity (`~/.rokha/anon_session`). A UUID, never anything sensitive.
fn anon_session_id() -> String {
    let path = dirs::home_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join(".rokha")
        .join("anon_session");
    if let Ok(s) = std::fs::read_to_string(&path) {
        let s = s.trim().to_string();
        if uuid::Uuid::parse_str(&s).is_ok() {
            return s;
        }
    }
    let id = uuid::Uuid::new_v4().to_string();
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let _ = std::fs::write(&path, &id);
    id
}

/// Read the dispatch acknowledgement; on a denial print it LOUDLY with the
/// honest ladder up (the terminal twin of the allowance banner).
async fn read_ack(
    resp: Result<reqwest::Response, reqwest::Error>,
    t: Theme,
) -> Result<String, i32> {
    let resp = match resp {
        Ok(r) => r,
        Err(e) => {
            eprintln!("could not reach the runtime: {e}");
            return Err(1);
        }
    };
    let status = resp.status();
    let body: Value = resp.json().await.unwrap_or_else(|_| json!({}));
    if status.is_success() {
        match body.get("run_id").and_then(|v| v.as_str()) {
            Some(id) => Ok(id.to_string()),
            None => {
                eprintln!("dispatch answered without a run_id: {body}");
                Err(1)
            }
        }
    } else {
        let msg = body
            .get("message")
            .and_then(|v| v.as_str())
            .unwrap_or("the runtime refused the run");
        eprintln!("{} {msg}", t.amber("⚠"));
        if status.as_u16() == 429 || status.as_u16() == 402 {
            eprintln!(
                "{}",
                t.amber("  That's a plan limit. Resets on the daily cycle, or:")
            );
            eprintln!("    • log in (`ro login`) to run on your own quota");
            eprintln!("    • upgrade / top up → https://rokha.ai/?plan=1");
        }
        Err(1)
    }
}

/// Poll the public progress door, printing each new stage as it lands.
/// Returns the final status once `done`, or None on poll-window timeout.
async fn poll_progress(
    http: &reqwest::Client,
    base: &str,
    run_id: &str,
    t: Theme,
) -> Option<String> {
    let arrow = if t.unicode { "\u{25b8}" } else { ">" };
    let mut seen = 0usize;
    for _ in 0..MAX_POLLS {
        tokio::time::sleep(std::time::Duration::from_millis(POLL_MS)).await;
        let snap: Value = match http
            .get(format!("{base}/api/runtime/runs/{run_id}/progress"))
            .send()
            .await
        {
            Ok(r) => r.json().await.unwrap_or_else(|_| json!({})),
            Err(_) => continue, // transient network blip — keep polling
        };
        if let Some(stages) = snap.get("progress").and_then(|v| v.as_array()) {
            for ev in stages.iter().skip(seen) {
                let stage = ev.get("stage").and_then(|v| v.as_str()).unwrap_or("");
                let msg = ev.get("msg").and_then(|v| v.as_str()).unwrap_or("");
                println!("{}", t.dim(&format!("{arrow} {stage}  {msg}")));
                let _ = std::io::stdout().flush();
            }
            seen = stages.len();
        }
        if snap.get("done").and_then(|v| v.as_bool()).unwrap_or(false) {
            return Some(
                snap.get("status")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown")
                    .to_string(),
            );
        }
    }
    None
}

/// Best-effort receipt read-back for anon runs: the trace the run wrote,
/// fetched through the anon rail. A miss is quiet — the run already reported
/// its status honestly above.
async fn print_anon_trace(
    http: &reqwest::Client,
    base: &str,
    session: &str,
    harness_id: &str,
    t: Theme,
) {
    let body: Value = match http
        .get(format!(
            "{base}/api/anon/traces?harness_id={harness_id}&limit=3"
        ))
        .header("x-anon-session-id", session)
        .send()
        .await
    {
        Ok(r) => r.json().await.unwrap_or_else(|_| json!({})),
        Err(_) => return,
    };
    let Some(trace) = body
        .get("data")
        .and_then(|v| v.as_array())
        .and_then(|a| a.first())
    else {
        return;
    };
    let result = trace.get("result").cloned().unwrap_or(Value::Null);
    let text = result
        .get("response")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| serde_json::to_string_pretty(&result).unwrap_or_default());
    let text = text.trim();
    if text.is_empty() || text == "null" {
        return;
    }
    println!();
    println!("{}", t.ice("── the run's output ──"));
    // Cap terminal spam; the full payload lives in the trace.
    const CAP: usize = 4000;
    if text.chars().count() > CAP {
        let cut: String = text.chars().take(CAP).collect();
        println!("{cut}");
        println!("{}", t.faint("… (truncated — full payload in the trace)"));
    } else {
        println!("{text}");
    }
}
