//! Paid-only gate for the local agent (founder, 2026-07-15).
//!
//! The local Rokha agent is a paid feature. The value commands (`ro agent`,
//! `ro voice`) require a logged-in **paid** profile; anon/free is refused
//! LOUDLY with the upgrade path (the golden rule: block loudly, upsell
//! honestly). Login/status/version stay open so a user can actually sign in.
//!
//! This is a UX gate, not a security boundary — the server enforces tier on
//! every door regardless. We fail fast here so a free user gets the honest
//! "here's how to unlock it" instead of a raw 402/403 from the wire.

use crate::credentials::{self, Credentials};

const PLANS_URL: &str = "https://rokha.ai/?plan=1";

/// Tiers that unlock the local agent. `free` and unknown tiers do not.
fn is_paid(tier: &str) -> bool {
    matches!(tier, "casual" | "pro")
}

/// Require a logged-in paid profile. On success returns the credentials; on
/// failure prints the loud upsell and returns the process exit code to use.
pub fn require_paid(feature: &str) -> Result<Credentials, i32> {
    match credentials::load() {
        None => {
            need_login(feature);
            Err(2)
        }
        Some(creds) if !is_paid(&creds.identity.tier) => {
            need_upgrade(feature, &creds.identity.tier);
            Err(2)
        }
        Some(creds) => Ok(creds),
    }
}

fn need_login(feature: &str) {
    eprintln!();
    eprintln!("\x1b[1m🔒 {feature} is part of the local Rokha agent — a paid feature.\x1b[0m");
    eprintln!();
    eprintln!("  You're not logged in. To use it:");
    eprintln!("    1. \x1b[1mro login\x1b[0m        — sign in through your browser");
    eprintln!("    2. be on a paid plan (Casual or Pro)");
    eprintln!();
    eprintln!("  See plans → {PLANS_URL}");
    eprintln!();
}

fn need_upgrade(feature: &str, tier: &str) {
    eprintln!();
    eprintln!("\x1b[1m🔒 {feature} is a paid feature — your plan is '{tier}'.\x1b[0m");
    eprintln!();
    eprintln!("  The local Rokha agent unlocks on \x1b[1mCasual\x1b[0m or \x1b[1mPro\x1b[0m.");
    eprintln!("  Upgrade → {PLANS_URL}");
    eprintln!();
}
