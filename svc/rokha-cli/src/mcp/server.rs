use crate::api_client::RokhaClient;
use crate::credentials;
use std::io::{self, BufRead, Write};

/// `ro mcp serve` — a thin local MCP bridge over stdio.
///
/// This is the seed of the local Rokha agent: a lightweight local interface
/// that does *no* heavy lifting itself. Every JSON-RPC envelope a client
/// (Claude Desktop, Claude Code, any MCP host) writes to stdin is forwarded
/// verbatim to the platform's real MCP endpoint (`{base}/mcp/jsonrpc`) — the
/// registry search, skill fetch, chat, rig, and run tools all live and execute
/// on the platform. The response is written back unchanged.
///
/// If the user is logged in (`ro login`), the stored JWT is attached so the
/// authed tool set is available; otherwise the public tool set answers.
///
/// Because it is a transparent passthrough, new platform tools appear here the
/// moment the server ships them — no CLI release required.
pub async fn serve(client: &RokhaClient) {
    let creds = credentials::load();
    let jwt = creds.as_ref().map(|c| c.jwt.as_str());

    eprintln!(
        "Rokha MCP bridge → {} ({})",
        client.base_url(),
        if jwt.is_some() {
            "authenticated"
        } else {
            "public tool set — run `ro login` for the full toolkit"
        }
    );

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };
        if line.trim().is_empty() {
            continue;
        }

        let req: serde_json::Value = match serde_json::from_str(&line) {
            Ok(v) => v,
            Err(_) => continue, // not JSON — ignore, stay alive
        };

        // JSON-RPC notifications carry no `id` and MUST NOT be answered. We
        // still forward them best-effort (so the platform sees the lifecycle),
        // but write nothing back.
        let is_notification = req.get("id").is_none();
        let id = req.get("id").cloned().unwrap_or(serde_json::Value::Null);

        let response = match client.mcp_forward(&req, jwt).await {
            Ok(v) => v,
            Err(e) => serde_json::json!({
                "jsonrpc": "2.0",
                "id": id,
                "error": {
                    "code": -32000,
                    "message": format!("Rokha bridge could not reach the platform: {e}")
                }
            }),
        };

        if is_notification {
            continue;
        }

        if let Ok(out) = serde_json::to_string(&response) {
            let _ = writeln!(stdout, "{out}");
            let _ = stdout.flush();
        }
    }
}
