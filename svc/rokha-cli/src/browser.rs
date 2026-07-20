//! Browser-control plane for the local agent (Phase C).
//!
//! `ro` launches or attaches a local Chrome/Chromium at rokha.ai with remote
//! debugging, then drives it over the Chrome DevTools Protocol (CDP): it can
//! navigate the page and forward the agent's `ui_directives` into it.
//!
//! Public/private split, upheld: `ro` holds NO HUD logic. It fires ONE
//! `rokha-remote-directives` window event carrying the raw directive array; the
//! rokha.ai page's own dispatcher (which already knows every directive → view
//! mapping) applies them. So "open the registry rail / put SKILL STATS up",
//! spoken to the terminal, steers the real browser — and new directive types
//! work with no CLI release.

use futures_util::{SinkExt, StreamExt};
use serde_json::{json, Value};
use std::time::Duration;
use tokio_tungstenite::tungstenite::Message;

const DEBUG_PORT: u16 = 9222;

type WsStream =
    tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>;

pub struct Browser {
    ws: WsStream,
    next_id: i64,
}

impl Browser {
    /// Ensure a Chrome with remote debugging is up (launch it if not), then open
    /// a CDP session to its first page target. Idempotent — re-attaches to an
    /// already-running instance rather than spawning a second.
    pub async fn launch_or_attach(start_url: &str) -> Result<Browser, String> {
        let http = crate::api_client::http_client();

        if !debugger_up(&http).await {
            spawn_chrome(start_url)?;
            // Wait up to ~10s for the debugger endpoint to answer.
            let mut up = false;
            for _ in 0..50 {
                tokio::time::sleep(Duration::from_millis(200)).await;
                if debugger_up(&http).await {
                    up = true;
                    break;
                }
            }
            if !up {
                return Err("Chrome launched but its remote-debugging port never came up".into());
            }
        }

        let ws_url = page_ws_url(&http).await?;
        let (ws, _) = tokio_tungstenite::connect_async(&ws_url)
            .await
            .map_err(|e| format!("could not open a CDP session: {e}"))?;
        let mut browser = Browser { ws, next_id: 1 };
        // Enable the Page domain so navigation events work; ignore the ack.
        let _ = browser.call("Page.enable", json!({})).await;
        Ok(browser)
    }

    /// Navigate the controlled page to `url`.
    pub async fn navigate(&mut self, url: &str) -> Result<(), String> {
        self.call("Page.navigate", json!({ "url": url })).await.map(|_| ())
    }

    /// Forward the agent's `ui_directives` into the page as a single
    /// `rokha-remote-directives` event. The page's own dispatcher applies them.
    pub async fn apply_directives(&mut self, directives: &Value) -> Result<(), String> {
        // `directives` is already valid JSON; embed it directly into the event.
        let expr = format!(
            "window.dispatchEvent(new CustomEvent('rokha-remote-directives', \
             {{ detail: {{ directives: {directives} }} }}))"
        );
        self.call(
            "Runtime.evaluate",
            json!({ "expression": expr, "awaitPromise": false, "returnByValue": true }),
        )
        .await
        .map(|_| ())
    }

    /// Send one CDP command and read frames until the matching response id
    /// arrives (events / other ids are skipped). Sequential request/response —
    /// fine for our one-command-at-a-time driving.
    async fn call(&mut self, method: &str, params: Value) -> Result<Value, String> {
        let id = self.next_id;
        self.next_id += 1;
        let msg = json!({ "id": id, "method": method, "params": params }).to_string();
        self.ws
            .send(Message::Text(msg))
            .await
            .map_err(|e| format!("CDP send failed: {e}"))?;

        while let Some(next) = self.ws.next().await {
            let frame = next.map_err(|e| format!("CDP receive failed: {e}"))?;
            if let Message::Text(txt) = frame {
                let v: Value = serde_json::from_str(&txt).unwrap_or(Value::Null);
                if v.get("id").and_then(|i| i.as_i64()) == Some(id) {
                    if let Some(err) = v.get("error") {
                        return Err(format!("CDP error: {err}"));
                    }
                    return Ok(v.get("result").cloned().unwrap_or(Value::Null));
                }
                // otherwise: an event or a different id — keep reading.
            }
        }
        Err("CDP connection closed before a response arrived".into())
    }
}

async fn debugger_up(http: &reqwest::Client) -> bool {
    http.get(format!("http://127.0.0.1:{DEBUG_PORT}/json/version"))
        .send()
        .await
        .map(|r| r.status().is_success())
        .unwrap_or(false)
}

/// The `webSocketDebuggerUrl` of the first `page` target.
async fn page_ws_url(http: &reqwest::Client) -> Result<String, String> {
    let list: Value = http
        .get(format!("http://127.0.0.1:{DEBUG_PORT}/json"))
        .send()
        .await
        .map_err(|e| format!("could not list Chrome targets: {e}"))?
        .json()
        .await
        .map_err(|e| format!("could not parse Chrome targets: {e}"))?;
    list.as_array()
        .and_then(|a| {
            a.iter()
                .find(|t| t.get("type").and_then(|v| v.as_str()) == Some("page"))
        })
        .and_then(|t| t.get("webSocketDebuggerUrl").and_then(|v| v.as_str()))
        .map(str::to_string)
        .ok_or_else(|| "no page tab found in the running Chrome".into())
}

fn spawn_chrome(start_url: &str) -> Result<(), String> {
    let bin = find_chrome()
        .ok_or("could not find Chrome/Chromium — install Google Chrome to use browser control")?;
    let dir = dirs::home_dir()
        .unwrap_or_default()
        .join(".rokha")
        .join("browser");
    let _ = std::fs::create_dir_all(&dir);
    std::process::Command::new(bin)
        .arg(format!("--remote-debugging-port={DEBUG_PORT}"))
        .arg(format!("--user-data-dir={}", dir.display()))
        .arg("--no-first-run")
        .arg("--no-default-browser-check")
        .arg(start_url)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .map_err(|e| format!("could not launch Chrome: {e}"))?;
    Ok(())
}

fn find_chrome() -> Option<String> {
    let candidates: &[&str] = if cfg!(target_os = "macos") {
        &[
            "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
            "/Applications/Chromium.app/Contents/MacOS/Chromium",
        ]
    } else {
        &[
            "google-chrome-stable",
            "google-chrome",
            "chromium",
            "chromium-browser",
            "chrome",
        ]
    };
    for c in candidates {
        if c.contains('/') {
            if std::path::Path::new(c).exists() {
                return Some((*c).to_string());
            }
        } else if on_path(c) {
            return Some((*c).to_string());
        }
    }
    None
}

fn on_path(cmd: &str) -> bool {
    std::env::var_os("PATH")
        .map(|paths| std::env::split_paths(&paths).any(|p| p.join(cmd).is_file()))
        .unwrap_or(false)
}
