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
use crate::theme::{self, Theme};
use std::cell::Cell;
use std::io::Write;
use tokio::io::{AsyncBufReadExt, BufReader};

pub async fn repl(client: &RokhaClient, use_browser: bool) -> i32 {
    theme::banner("the local agent");
    let t = Theme::detect();
    let creds = match gate::require_paid("ro agent") {
        Ok(c) => c,
        Err(code) => return code,
    };
    let http = crate::api_client::http_client();

    // Browser-control plane: attach a local Chrome at rokha.ai so Rokha can
    // steer its views from the terminal. Optional — a failure never blocks chat.
    let mut browser = if use_browser {
        match crate::browser::Browser::launch_or_attach("https://rokha.ai").await {
            Ok(b) => {
                println!("\x1b[2m(browser control on — Rokha can steer rokha.ai)\x1b[0m");
                Some(b)
            }
            Err(e) => {
                eprintln!("\x1b[2m(browser control unavailable: {e})\x1b[0m");
                None
            }
        }
    } else {
        None
    };

    println!(
        "Connected as {} {}",
        t.ice_bold(&creds.identity.identity),
        t.dim(&format!("({})", creds.identity.tier))
    );
    println!(
        "{}",
        t.faint("Type a message and press enter · /exit to quit · /help for commands")
    );
    println!();

    let mut lines = BufReader::new(tokio::io::stdin()).lines();
    loop {
        print!("{}", t.prompt());
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
                print_help(&t);
                continue;
            }
            _ => {}
        }
        run_turn(
            &http,
            client.base_url(),
            &creds.jwt,
            msg,
            browser.as_mut(),
            t,
        )
        .await;
    }
    println!("{}", t.dim("seeds away."));
    0
}

/// Stream a single turn and render it live. Returns nothing — errors are printed
/// inline (with a loud upsell when the failure is a tier/allowance limit).
async fn run_turn(
    http: &reqwest::Client,
    base: &str,
    jwt: &str,
    message: &str,
    browser: Option<&mut crate::browser::Browser>,
    t: Theme,
) {
    // `Cell` so the FnMut render closure can mutate render state without an
    // exclusive borrow leaking past the await.
    let header_shown = Cell::new(false);
    let streamed = Cell::new(false);
    // HUD directives collected from the turn's canonical `content` event.
    let directives: std::cell::RefCell<Option<serde_json::Value>> = std::cell::RefCell::new(None);

    let seed = if t.unicode { "\u{273f}" } else { "*" }; // ✿ — Rokha's speaking mark
    let arrow = if t.unicode { "\u{25b8}" } else { ">" };
    let render = |evt: ChatEvent| match evt {
        ChatEvent::ContentStart => {}
        ChatEvent::ContentDelta { text } => {
            if !header_shown.get() {
                print!("{}  ", t.ice_bold(seed));
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
            println!("{}", t.dim(&format!("{arrow} {tool}\u{2026}")));
            header_shown.set(false); // the post-tool answer re-prints the prefix
        }
        ChatEvent::ToolResult { tool, error } => {
            if let Some(err) = error {
                println!("{}", t.dim(&format!("  \u{21b3} {tool} failed: {err}")));
            }
        }
        ChatEvent::Content { json } => {
            // Canonical turn. If deltas already streamed, this is a no-op; if the
            // server didn't stream (buffered fallback), print it now.
            if !streamed.get() {
                let text = stream::content_text(&json);
                if !text.is_empty() {
                    print!("{}  {text}", t.ice_bold(seed));
                }
            }
            // Capture any HUD steering to forward to the browser after the turn.
            if let Some(dirs) = stream::ui_directives(&json) {
                *directives.borrow_mut() = Some(dirs);
            }
        }
        ChatEvent::Error { message, status } => {
            println!();
            eprintln!("\x1b[31m⚠ {message}\x1b[0m");
            maybe_upsell(status, &message, t);
        }
        ChatEvent::Done => {}
    };

    let res = stream::stream_chat(http, base, Some(jwt), message, render).await;
    println!(); // terminate the reply line

    // Browser-control: forward any HUD directives into the attached page.
    if let (Some(b), Some(dirs)) = (browser, directives.into_inner()) {
        if let Err(e) = b.apply_directives(&dirs).await {
            eprintln!("\x1b[2m(browser steer failed: {e})\x1b[0m");
        }
    }

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
        maybe_upsell(status, &e, t);
    }
}

/// Loud, honest upsell when a turn is denied on a tier/allowance limit — the
/// terminal twin of the in-app allowance banner.
fn maybe_upsell(status: u64, message: &str, t: Theme) {
    let m = message.to_lowercase();
    let limit_shaped = status == 402
        || status == 429
        || m.contains("limit")
        || m.contains("quota")
        || m.contains("budget")
        || m.contains("allowance")
        || m.contains("upgrade");
    if limit_shaped {
        eprintln!(
            "{} Resets on your daily cycle, or:",
            t.amber("  You hit a plan limit.")
        );
        eprintln!("    • upgrade / top up → https://rokha.ai/?plan=1");
    }
}

fn print_help(t: &Theme) {
    println!(
        "{}",
        t.faint("  /exit, /quit   end the session\n  /help          this list\n  (anything else is sent to Rokha)")
    );
}
