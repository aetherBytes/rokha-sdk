//! `ro voice` — talk to Rokha in the terminal (feature = "voice").
//!
//! A phone-call-with-your-agent loop: press Enter, speak, pause, and Rokha
//! replies aloud — streaming her text live while she talks. Thin by design: the
//! mic and speaker are local; STT, the agent turn, and TTS all run on the
//! platform. Paid-gated (the local agent is a paid feature), and every limit
//! denial upsells loudly.

use crate::api_client::{RokhaClient, VoiceError};
use crate::audio;
use crate::gate;
use crate::stream::{self, ChatEvent};
use std::cell::{Cell, RefCell};
use std::io::Write;
use tokio::io::{AsyncBufReadExt, BufReader};

pub async fn convo(client: &RokhaClient, use_browser: bool) -> i32 {
    let creds = match gate::require_paid("ro voice") {
        Ok(c) => c,
        Err(code) => return code,
    };
    let http = crate::api_client::http_client();

    // Browser-control plane (optional): Rokha can steer rokha.ai as you talk.
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

    // Honest availability check — never pretend voice works when it doesn't.
    match client.voice_health().await {
        Ok(h) => {
            let stt = h.get("stt").and_then(|v| v.as_bool()).unwrap_or(false);
            let tts = h.get("tts").and_then(|v| v.as_bool()).unwrap_or(false);
            if !stt || !tts {
                eprintln!("Voice isn't available on this server right now (stt={stt}, tts={tts}).");
                eprintln!("Use `ro agent` for text chat in the meantime.");
                return 1;
            }
        }
        Err(e) => {
            eprintln!("Could not reach the voice service: {e}");
            return 1;
        }
    }

    println!();
    println!(
        "\x1b[1m◉ LIVE VOICE\x1b[0m — connected as \x1b[1m{}\x1b[0m ({}).",
        creds.identity.identity, creds.identity.tier
    );
    println!("Press \x1b[1mEnter\x1b[0m to talk; pause when you're done and Rokha replies aloud.");
    println!("\x1b[2mCtrl-C to end.\x1b[0m");
    println!();

    let mut lines = BufReader::new(tokio::io::stdin()).lines();
    loop {
        print!("\x1b[1m🎙  Enter to talk ›\x1b[0m ");
        let _ = std::io::stdout().flush();

        // Wait for Enter, but let Ctrl-C end the session cleanly.
        tokio::select! {
            _ = tokio::signal::ctrl_c() => { println!("\nended."); break; }
            line = lines.next_line() => {
                match line {
                    Ok(Some(_)) => {}     // Enter — proceed to capture
                    Ok(None) => break,     // EOF (Ctrl-D)
                    Err(e) => { eprintln!("input error: {e}"); break; }
                }
            }
        }

        // 1. Capture one utterance (blocking audio → its own thread).
        println!("\x1b[2m🎤 listening… (pause when done)\x1b[0m");
        let wav = match tokio::task::spawn_blocking(audio::record_utterance).await {
            Ok(Ok(w)) => w,
            Ok(Err(e)) => {
                eprintln!("\x1b[2m({e})\x1b[0m");
                continue;
            }
            Err(e) => {
                eprintln!("capture task failed: {e}");
                continue;
            }
        };

        // 2. STT on the platform.
        let transcript = match client.voice_stt(wav, &creds.jwt).await {
            Ok(t) => t,
            Err(e) => {
                report_voice_err(&e);
                continue;
            }
        };
        let transcript = transcript.trim().to_string();
        if transcript.is_empty() {
            println!("\x1b[2m(didn't catch that — try again)\x1b[0m");
            continue;
        }
        println!("\x1b[1myou\x1b[0m  {transcript}");

        // 3. Agent turn — stream text live, collect the full reply to speak.
        let reply = RefCell::new(String::new());
        let streamed = Cell::new(false);
        let header = Cell::new(false);
        let directives: RefCell<Option<serde_json::Value>> = RefCell::new(None);
        let render = |evt: ChatEvent| match evt {
            ChatEvent::ContentDelta { text } => {
                if !header.get() {
                    print!("\x1b[1mRo\x1b[0m  ");
                    header.set(true);
                }
                print!("{text}");
                let _ = std::io::stdout().flush();
                reply.borrow_mut().push_str(&text);
                streamed.set(true);
            }
            ChatEvent::ToolStart { tool } => {
                if header.get() {
                    println!();
                }
                println!("\x1b[2m▸ {tool}…\x1b[0m");
                header.set(false);
            }
            ChatEvent::Content { json } => {
                if !streamed.get() {
                    let text = stream::content_text(&json);
                    if !text.is_empty() {
                        print!("\x1b[1mRo\x1b[0m  {text}");
                        *reply.borrow_mut() = text;
                    }
                }
                if let Some(dirs) = stream::ui_directives(&json) {
                    *directives.borrow_mut() = Some(dirs);
                }
            }
            ChatEvent::Error { message, .. } => {
                println!();
                eprintln!("\x1b[31m⚠ {message}\x1b[0m");
            }
            _ => {}
        };
        let res = stream::stream_chat(&http, client.base_url(), Some(&creds.jwt), &transcript, render).await;
        println!();
        if let Err(e) = res {
            eprintln!("\x1b[31m⚠ {e}\x1b[0m");
            continue;
        }

        // 3b. Forward any HUD steering into the attached browser.
        if let (Some(b), Some(dirs)) = (browser.as_mut(), directives.into_inner()) {
            if let Err(e) = b.apply_directives(&dirs).await {
                eprintln!("\x1b[2m(browser steer failed: {e})\x1b[0m");
            }
        }

        // 4. Speak the reply.
        let text = reply.into_inner();
        speak(client, &creds.jwt, &text).await;
    }
    0
}

/// TTS the reply and play it, chunk by chunk (respecting the door's 2000-char
/// cap and giving faster time-to-first-audio). Stops on the first failure.
async fn speak(client: &RokhaClient, jwt: &str, text: &str) {
    let clean = speakable(text);
    if clean.trim().is_empty() {
        return;
    }
    for chunk in chunk_sentences(&clean, 1500) {
        match client.voice_tts(&chunk, jwt).await {
            Ok(mp3) => {
                let played = tokio::task::spawn_blocking(move || audio::play_mp3(mp3)).await;
                match played {
                    Ok(Ok(())) => {}
                    Ok(Err(e)) => {
                        eprintln!("\x1b[2m(playback: {e})\x1b[0m");
                        break;
                    }
                    Err(e) => {
                        eprintln!("\x1b[2m(playback task: {e})\x1b[0m");
                        break;
                    }
                }
            }
            Err(e) => {
                report_voice_err(&e);
                break;
            }
        }
    }
}

/// Print a voice-door error, upselling loudly when it's a tier/allowance limit.
fn report_voice_err(e: &VoiceError) {
    if e.is_unconfigured() {
        eprintln!("\x1b[2m(voice not configured on the server: {})\x1b[0m", e.message);
        return;
    }
    eprintln!("\x1b[31m⚠ {}\x1b[0m", e.message);
    if e.is_limit() {
        eprintln!("\x1b[1m  Voice allowance reached.\x1b[0m Resets on your daily cycle, or:");
        eprintln!("    • upgrade / top up voice → https://rokha.ai/?plan=1");
    }
}

/// Strip the markdown Rokha writes for the eye so the ear hears clean prose.
fn speakable(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut in_code_fence = false;
    for line in text.lines() {
        if line.trim_start().starts_with("```") {
            in_code_fence = !in_code_fence;
            continue;
        }
        if in_code_fence {
            continue;
        }
        let stripped: String = line
            .chars()
            .filter(|c| !matches!(c, '*' | '`' | '#' | '_' | '>' | '|'))
            .collect();
        out.push_str(stripped.trim_end());
        out.push('\n');
    }
    out
}

/// Group sentences into chunks no longer than `max` chars, breaking on sentence
/// boundaries where possible so the vendor never gets a mid-word cut.
fn chunk_sentences(text: &str, max: usize) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut current = String::new();
    for sentence in split_sentences(text) {
        if current.len() + sentence.len() > max && !current.is_empty() {
            chunks.push(std::mem::take(&mut current));
        }
        if sentence.len() > max {
            // A single very long sentence: hard-split on char boundaries.
            for piece in sentence.as_bytes().chunks(max) {
                chunks.push(String::from_utf8_lossy(piece).to_string());
            }
        } else {
            current.push_str(&sentence);
        }
    }
    if !current.trim().is_empty() {
        chunks.push(current);
    }
    chunks
}

fn split_sentences(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    for ch in text.chars() {
        cur.push(ch);
        if matches!(ch, '.' | '!' | '?' | '\n') {
            let trimmed = cur.trim();
            if !trimmed.is_empty() {
                out.push(format!("{} ", trimmed));
            }
            cur.clear();
        }
    }
    if !cur.trim().is_empty() {
        out.push(cur.trim().to_string());
    }
    out
}
