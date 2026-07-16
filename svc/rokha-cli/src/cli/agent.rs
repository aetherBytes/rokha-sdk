//! `ro agent` — the interactive REPL over the platform Rokha agent.
//!
//! This is the spine of the local agent: a conversational loop that streams the
//! same server-side Rokha (LLM, personality, tool loop, memory, billing) you get
//! in the browser. Thin by design — we render tokens and tool events; the
//! platform does all the thinking. Paid-gated (see `gate`). `ro voice` wraps an
//! audio I/O layer around this exact loop.

use crate::api_client::RokhaClient;
use crate::gate;
use crate::stream::{self, ChatEvent};
use std::cell::Cell;
use std::io::Write;
use tokio::io::{AsyncBufReadExt, BufReader};

pub async fn repl(client: &RokhaClient) -> i32 {
    let creds = match gate::require_paid("ro agent") {
        Ok(c) => c,
        Err(code) => return code,
    };
    let http = crate::api_client::http_client();

    println!();
    println!(
        "\x1b[1mRokha\x1b[0m — local agent. Connected as \x1b[1m{}\x1b[0m ({}).",
        creds.identity.identity, creds.identity.tier
    );
    println!("Type a message and press enter. \x1b[2m/exit to quit · /help for commands\x1b[0m");
    println!();

    let mut lines = BufReader::new(tokio::io::stdin()).lines();
    loop {
        print!("\x1b[1m› \x1b[0m");
        let _ = std::io::stdout().flush();

        let line = match lines.next_line().await {
            Ok(Some(l)) => l,
            Ok(None) => break, // EOF / Ctrl-D
            Err(e) => {
                eprintln!("input error: {e}");
                break;
            }
        };
        let msg = line.trim();
        if msg.is_empty() {
            continue;
        }
        match msg {
            "/exit" | "/quit" | ":q" => break,
            "/help" => {
                print_help();
                continue;
            }
            _ => {}
        }
        run_turn(&http, client.base_url(), &creds.jwt, msg).await;
    }
    println!("bye.");
    0
}

/// Stream a single turn and render it live. Returns nothing — errors are printed
/// inline (with a loud upsell when the failure is a tier/allowance limit).
async fn run_turn(http: &reqwest::Client, base: &str, jwt: &str, message: &str) {
    // `Cell` so the FnMut render closure can mutate render state without an
    // exclusive borrow leaking past the await.
    let header_shown = Cell::new(false);
    let streamed = Cell::new(false);

    let render = |evt: ChatEvent| match evt {
        ChatEvent::ContentStart => {}
        ChatEvent::ContentDelta { text } => {
            if !header_shown.get() {
                print!("\x1b[1mRo\x1b[0m  ");
                header_shown.set(true);
            }
            print!("{text}");
            let _ = std::io::stdout().flush();
            streamed.set(true);
        }
        ChatEvent::ToolStart { tool } => {
            // Break the current line so tool notes don't collide with tokens.
            if header_shown.get() {
                println!();
            }
            println!("\x1b[2m▸ {tool}…\x1b[0m");
            header_shown.set(false); // the post-tool answer re-prints the prefix
        }
        ChatEvent::ToolResult { tool, error } => {
            if let Some(err) = error {
                println!("\x1b[2m  ↳ {tool} failed: {err}\x1b[0m");
            }
        }
        ChatEvent::Content { json } => {
            // Canonical turn. If deltas already streamed, this is a no-op; if the
            // server didn't stream (buffered fallback), print it now.
            if !streamed.get() {
                let text = stream::content_text(&json);
                if !text.is_empty() {
                    print!("\x1b[1mRo\x1b[0m  {text}");
                }
            }
        }
        ChatEvent::Error { message, status } => {
            println!();
            eprintln!("\x1b[31m⚠ {message}\x1b[0m");
            maybe_upsell(status, &message);
        }
        ChatEvent::Done => {}
    };

    let res = stream::stream_chat(http, base, Some(jwt), message, render).await;
    println!(); // terminate the reply line
    if let Err(e) = res {
        eprintln!("\x1b[31m⚠ {e}\x1b[0m");
        // Transport-level errors carry the HTTP status inside the string.
        let status = if e.contains("HTTP 402") {
            402
        } else if e.contains("HTTP 429") {
            429
        } else {
            0
        };
        maybe_upsell(status, &e);
    }
}

/// Loud, honest upsell when a turn is denied on a tier/allowance limit — the
/// terminal twin of the in-app allowance banner.
fn maybe_upsell(status: u64, message: &str) {
    let m = message.to_lowercase();
    let limit_shaped = status == 402
        || status == 429
        || m.contains("limit")
        || m.contains("quota")
        || m.contains("budget")
        || m.contains("allowance")
        || m.contains("upgrade");
    if limit_shaped {
        eprintln!("\x1b[1m  You hit a plan limit.\x1b[0m Resets on your daily cycle, or:");
        eprintln!("    • upgrade / top up → https://rokha.ai/?plan=1");
    }
}

fn print_help() {
    println!("\x1b[2m  /exit, /quit   end the session");
    println!("  /help          this list");
    println!("  (anything else is sent to Rokha)\x1b[0m");
}
