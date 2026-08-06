//! `ro mcp install [host]` — hook the Rokha MCP bridge into a local MCP host
//! with one command, so "discover → invoke" needs no hand-edited JSON.
//!
//! Hosts:
//! - `claude-code`    → drives `claude mcp add` (user scope) if the CLI exists
//! - `claude-desktop` → edits claude_desktop_config.json in place (backs the
//!   old file up beside it first; never clobbers other servers)
//! - `print` (default) → prints the stanza + per-host one-liners for everything
//!   else
//!
//! The stanza points at THIS binary by absolute path — `ro` may not be on the
//! host app's PATH (GUI apps on macOS famously aren't shell-PATH aware).

use crate::theme::Theme;
use serde_json::{json, Value};
use std::path::PathBuf;
use std::process::Command;

fn ro_path() -> String {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.to_str().map(String::from))
        .unwrap_or_else(|| "ro".to_string())
}

fn stanza(ro: &str) -> Value {
    json!({ "command": ro, "args": ["mcp", "serve"] })
}

pub fn install(host: &str) -> i32 {
    let t = Theme::detect();
    let ro = ro_path();
    match host {
        "print" => {
            println!(
                "Add Rokha to any MCP host — the bridge serves the platform's full tool suite."
            );
            println!();
            println!("{}", t.ice("Claude Code:"));
            println!("  claude mcp add --scope user rokha -- {ro} mcp serve");
            println!();
            println!(
                "{}",
                t.ice("Claude Desktop (claude_desktop_config.json → mcpServers):")
            );
            println!(
                "{}",
                serde_json::to_string_pretty(&json!({ "rokha": stanza(&ro) })).unwrap_or_default()
            );
            println!();
            println!("{}", t.ice("Or let ro do it:"));
            println!("  ro mcp install claude-code");
            println!("  ro mcp install claude-desktop");
            println!();
            println!(
                "{}",
                t.faint("Logged in (`ro login`), the bridge serves your authed toolkit; anonymous gets the public set.")
            );
            0
        }
        "claude-code" => {
            let status = Command::new("claude")
                .args([
                    "mcp", "add", "--scope", "user", "rokha", "--", &ro, "mcp", "serve",
                ])
                .status();
            match status {
                Ok(s) if s.success() => {
                    println!("{} rokha added to Claude Code (user scope).", t.ok("✓"));
                    println!("{}", t.faint("Verify: claude mcp list"));
                    0
                }
                Ok(s) => {
                    eprintln!("`claude mcp add` exited with {s} — run it by hand:");
                    eprintln!("  claude mcp add --scope user rokha -- {ro} mcp serve");
                    1
                }
                Err(_) => {
                    eprintln!(
                        "The `claude` CLI isn't on PATH. Install Claude Code, or add by hand:"
                    );
                    eprintln!("  claude mcp add --scope user rokha -- {ro} mcp serve");
                    1
                }
            }
        }
        "claude-desktop" => match install_desktop(&ro) {
            Ok(path) => {
                println!(
                    "{} rokha added to Claude Desktop ({}).",
                    t.ok("✓"),
                    path.display()
                );
                println!("{}", t.faint("Restart Claude Desktop to pick it up."));
                0
            }
            Err(e) => {
                eprintln!("could not update Claude Desktop config: {e}");
                eprintln!("Add this to claude_desktop_config.json → mcpServers by hand:");
                eprintln!(
                    "{}",
                    serde_json::to_string_pretty(&json!({ "rokha": stanza(&ro) }))
                        .unwrap_or_default()
                );
                1
            }
        },
        other => {
            eprintln!("unknown host '{other}' — use claude-code, claude-desktop, or print");
            1
        }
    }
}

fn desktop_config_path() -> Option<PathBuf> {
    #[cfg(target_os = "macos")]
    {
        dirs::home_dir()
            .map(|h| h.join("Library/Application Support/Claude/claude_desktop_config.json"))
    }
    #[cfg(target_os = "linux")]
    {
        dirs::config_dir().map(|c| c.join("Claude/claude_desktop_config.json"))
    }
    #[cfg(target_os = "windows")]
    {
        dirs::config_dir().map(|c| c.join("Claude/claude_desktop_config.json"))
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        None
    }
}

/// Merge `mcpServers.rokha` into the desktop config, preserving everything
/// else. The previous file survives beside it as `.bak`.
fn install_desktop(ro: &str) -> Result<PathBuf, String> {
    let path = desktop_config_path().ok_or("unsupported platform")?;
    let mut root: Value = match std::fs::read_to_string(&path) {
        Ok(body) => serde_json::from_str(&body).map_err(|e| {
            format!("existing config is not valid JSON ({e}) — fix or remove it first")
        })?,
        Err(_) => json!({}),
    };
    if !root.is_object() {
        return Err("existing config root is not a JSON object".into());
    }
    if path.exists() {
        let bak = path.with_extension("json.bak");
        std::fs::copy(&path, &bak).map_err(|e| format!("backup failed: {e}"))?;
    }
    let servers = root
        .as_object_mut()
        .unwrap()
        .entry("mcpServers")
        .or_insert_with(|| json!({}));
    if !servers.is_object() {
        return Err("mcpServers is not a JSON object".into());
    }
    servers
        .as_object_mut()
        .unwrap()
        .insert("rokha".into(), stanza(ro));
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    std::fs::write(
        &path,
        serde_json::to_string_pretty(&root).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stanza_is_the_bridge_invocation() {
        let s = stanza("/usr/local/bin/ro");
        assert_eq!(s["command"], "/usr/local/bin/ro");
        assert_eq!(s["args"], json!(["mcp", "serve"]));
    }
}
