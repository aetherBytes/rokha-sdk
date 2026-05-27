use crate::api_client::RokhaClient;
use crate::credentials;
use serde_json::Value;

/// `ro chat <message>` — sends to rokha.ai's Rokha agent.
///
/// If logged in (~/.rokha/credentials present), uses the authenticated
/// endpoint `/api/agents/rokha-agent/chat` — full harness context, full
/// tool toolkit, response includes any tool_calls + tool_results that
/// fired during the turn.
///
/// If not logged in, falls back to `/api/agents/rokha-agent/chat/public` —
/// no harness, limited tools, pre-login tire-kicking only.
pub async fn chat(client: &RokhaClient, message: &str) -> i32 {
    let creds = credentials::load();
    let logged_in = creds.is_some();

    let body = serde_json::json!({ "message": message });
    let path = if logged_in {
        "/api/agents/rokha-agent/chat"
    } else {
        "/api/agents/rokha-agent/chat/public"
    };

    let url = format!("{}{}", client.base_url(), path);
    let mut req = reqwest::Client::new().post(&url).json(&body);
    if let Some(c) = &creds {
        req = req.bearer_auth(&c.jwt);
    }

    let res = match req.send().await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Chat failed: {}", e);
            return 1;
        }
    };

    let status = res.status();
    let json: Value = match res.json().await {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Malformed response: {}", e);
            return 1;
        }
    };

    if !status.is_success() {
        let msg = json
            .get("message")
            .and_then(|v| v.as_str())
            .unwrap_or("(no message)");
        eprintln!("HTTP {}: {}", status.as_u16(), msg);
        if status.as_u16() == 401 && logged_in {
            eprintln!("Session may be expired. Run `ro login` to refresh.");
        }
        return 1;
    }

    render_turn(&json);
    0
}

/// Render the agent's turn — text first, tool calls + results dim-formatted
/// below. Mirrors the shape of `/api/agents/rokha-agent/chat`'s response:
///   { "response": "<text>", "tool_calls": [...], "tool_results": [...], ... }
fn render_turn(json: &Value) {
    let text = json
        .get("response")
        .and_then(|v| v.as_str())
        .or_else(|| json.get("message").and_then(|v| v.as_str()))
        .unwrap_or("");

    if !text.is_empty() {
        println!("{}", text);
    }

    // Inline tool render. Server's response shape may carry these in a few
    // places depending on endpoint version; check both flat + nested.
    if let Some(tools) = json.get("tool_calls").and_then(|v| v.as_array()) {
        if !tools.is_empty() {
            eprintln!();
        }
        for tc in tools {
            let name = tc
                .get("function")
                .and_then(|f| f.get("name"))
                .and_then(|n| n.as_str())
                .or_else(|| tc.get("name").and_then(|n| n.as_str()))
                .unwrap_or("?");
            let args = tc
                .get("function")
                .and_then(|f| f.get("arguments"))
                .and_then(|a| a.as_str())
                .map(|s| compact_args(s))
                .unwrap_or_default();
            eprintln!("\x1b[2m▸ tool · {}({}){}\x1b[0m", name, args, "");
        }
    }
    if let Some(results) = json.get("tool_results").and_then(|v| v.as_array()) {
        for r in results {
            let preview = r
                .get("content")
                .and_then(|c| c.as_str())
                .map(|s| s.lines().next().unwrap_or("").to_string())
                .unwrap_or_default();
            if !preview.is_empty() {
                eprintln!("\x1b[2m  → {}\x1b[0m", truncate(&preview, 120));
            }
        }
    }
}

fn compact_args(json_str: &str) -> String {
    // Trim outer braces + whitespace for inline display. Full result lives in
    // the next line if the server included tool_results.
    let s = json_str.trim();
    if s == "{}" || s.is_empty() {
        String::new()
    } else {
        format!(" {}", s)
    }
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}…", &s[..max])
    }
}
