//! Thin SSE client for the Rokha agent's streaming chat door.
//!
//! Public/private split: this file carries NO intelligence. It POSTs a message
//! to `/api/agents/rokha-agent/chat/stream[/public]`, parses the Server-Sent
//! Events the platform emits, and hands each one to a callback. The agent brain
//! — tool loop, personality, billing — all runs server-side. We just carry the
//! bytes and render.
//!
//! Event shapes (mirrors `rokha-agents` `stream_chat_turn`):
//!   content_start  {}                         — a reply segment begins
//!   content_delta  { "text": "…" }            — a live token
//!   tool_start     { "tool": "…" }            — the loop invoked a tool
//!   tool_result    { "tool", "error", "latency_ms" }
//!   content        { …full ChatResponse… }    — canonical final turn (old
//!                                                clients handle only this)
//!   error          { "message", "status" }    — failure after the stream opened
//!   done           {}                          — end of turn

use futures_util::StreamExt;

#[derive(Debug, Clone)]
pub enum ChatEvent {
    ContentStart,
    ContentDelta { text: String },
    ToolStart { tool: String },
    ToolResult { tool: String, error: Option<String> },
    Content { json: serde_json::Value },
    Error { message: String, status: u64 },
    Done,
}

/// Extract the canonical reply text from a `content` event payload. The authed
/// door returns `response`; the public door returns `content`.
pub fn content_text(json: &serde_json::Value) -> String {
    json.get("content")
        .and_then(|v| v.as_str())
        .or_else(|| json.get("response").and_then(|v| v.as_str()))
        .or_else(|| json.get("message").and_then(|v| v.as_str()))
        .unwrap_or("")
        .to_string()
}

/// Stream one chat turn. Sends `{message, user_context:{}}` — erebus stamps the
/// real identity/tier onto `user_context` from the verified JWT, so we never
/// assert it. Calls `on_event` for every parsed SSE frame. Returns `Err` only on
/// a transport/HTTP failure that prevents the stream from opening; once the
/// stream is open, server-side failures arrive as a `ChatEvent::Error`.
pub async fn stream_chat<F: FnMut(ChatEvent)>(
    http: &reqwest::Client,
    base_url: &str,
    jwt: Option<&str>,
    message: &str,
    mut on_event: F,
) -> Result<(), String> {
    let path = if jwt.is_some() {
        "/api/agents/rokha-agent/chat/stream"
    } else {
        "/api/agents/rokha-agent/chat/stream/public"
    };
    let url = format!("{}{}", base_url.trim_end_matches('/'), path);

    let mut req = http
        .post(&url)
        .header(reqwest::header::ACCEPT, "text/event-stream")
        .json(&serde_json::json!({ "message": message, "user_context": {} }));
    if let Some(token) = jwt {
        req = req.bearer_auth(token);
    }

    let resp = req.send().await.map_err(|e| format!("request failed: {e}"))?;
    let status = resp.status();
    if !status.is_success() {
        // Read the body for a server message (limit err), then surface it.
        let body = resp.text().await.unwrap_or_default();
        let msg = serde_json::from_str::<serde_json::Value>(&body)
            .ok()
            .and_then(|v| {
                v.get("message")
                    .or_else(|| v.get("error"))
                    .and_then(|m| m.as_str())
                    .map(str::to_string)
            })
            .unwrap_or_else(|| body.lines().next().unwrap_or("").to_string());
        return Err(format!("HTTP {}: {}", status.as_u16(), msg));
    }

    let mut stream = resp.bytes_stream();
    let mut buf = String::new();

    while let Some(chunk) = stream.next().await {
        let bytes = chunk.map_err(|e| format!("stream read failed: {e}"))?;
        buf.push_str(&String::from_utf8_lossy(&bytes));

        // SSE records are separated by a blank line. Process every complete one.
        while let Some(idx) = buf.find("\n\n") {
            let record = buf[..idx].to_string();
            buf.drain(..idx + 2);
            if let Some(evt) = parse_record(&record) {
                let done = matches!(evt, ChatEvent::Done);
                on_event(evt);
                if done {
                    return Ok(());
                }
            }
        }
    }
    Ok(())
}

/// Parse one SSE record (already split on the blank-line boundary) into a
/// `ChatEvent`. Handles multi-line `data:` per the SSE spec and tolerates the
/// keep-alive comment frames (`:`), which parse to no event.
fn parse_record(record: &str) -> Option<ChatEvent> {
    let mut name: Option<&str> = None;
    let mut data = String::new();
    for line in record.lines() {
        if let Some(rest) = line.strip_prefix("event:") {
            name = Some(rest.trim());
        } else if let Some(rest) = line.strip_prefix("data:") {
            if !data.is_empty() {
                data.push('\n');
            }
            data.push_str(rest.strip_prefix(' ').unwrap_or(rest));
        }
        // lines starting with ':' are comments (keep-alive) — ignored.
    }
    let name = name?;
    let val = serde_json::from_str::<serde_json::Value>(&data).unwrap_or(serde_json::Value::Null);
    match name {
        "content_start" => Some(ChatEvent::ContentStart),
        "content_delta" => Some(ChatEvent::ContentDelta {
            text: val.get("text").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        }),
        "tool_start" => Some(ChatEvent::ToolStart {
            tool: val.get("tool").and_then(|v| v.as_str()).unwrap_or("?").to_string(),
        }),
        "tool_result" => Some(ChatEvent::ToolResult {
            tool: val.get("tool").and_then(|v| v.as_str()).unwrap_or("?").to_string(),
            error: val.get("error").and_then(|v| v.as_str()).map(str::to_string),
        }),
        "content" => Some(ChatEvent::Content { json: val }),
        "error" => Some(ChatEvent::Error {
            message: val.get("message").and_then(|v| v.as_str()).unwrap_or("stream error").to_string(),
            status: val.get("status").and_then(|v| v.as_u64()).unwrap_or(0),
        }),
        "done" => Some(ChatEvent::Done),
        _ => None,
    }
}
