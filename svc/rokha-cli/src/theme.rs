//! The Rokha terminal theme — "seeds on the wind".
//!
//! Rokha is Aramaic for breath/wind, and the platform mark is a seed. The
//! launch banner is a dandelion head breathing — inhale, exhale — while loose
//! seeds drift downwind toward the prompt. Ice-cyan on the terminal's own
//! background, the same Void HUD accent as rokha.ai.
//!
//! Thin-client rules apply to the theme too: zero dependencies, a few static
//! strings, and it degrades honestly — NO_COLOR / TERM=dumb / piped stdout get
//! plain text, non-UTF-8 locales get pure ASCII, `RO_NO_ANIM=1` (or any of the
//! above) skips the animation. Scriptable commands (`ro status`, `ro version`,
//! `ro tools`) never print the banner at all.

use std::io::{IsTerminal, Write};

#[derive(Clone, Copy, PartialEq)]
pub enum ColorMode {
    Truecolor,
    Ansi256,
    Plain,
}

#[derive(Clone, Copy)]
pub struct Theme {
    pub mode: ColorMode,
    pub unicode: bool,
}

// The Void HUD palette (hecate `_palette.scss` accents), with xterm-256
// fallbacks chosen by eye against the same swatches.
const ICE: (u8, u8, u8) = (111, 212, 234); // #6fd4ea → 117
const ICE_DIM: (u8, u8, u8) = (62, 168, 199); // #3ea8c7 → 74
const SLATE: (u8, u8, u8) = (110, 131, 145); // #6e8391 → 66
const FAINT: (u8, u8, u8) = (61, 82, 96); // #3d5260 → 240
const AMBER: (u8, u8, u8) = (224, 176, 96); // #e0b060 → 179
const GREEN: (u8, u8, u8) = (99, 207, 150); // #63cf96 → 78

fn idx256(rgb: (u8, u8, u8)) -> u8 {
    match rgb {
        ICE => 117,
        ICE_DIM => 74,
        SLATE => 66,
        FAINT => 240,
        AMBER => 179,
        GREEN => 78,
        _ => 117,
    }
}

impl Theme {
    pub fn detect() -> Self {
        let plain = std::env::var_os("NO_COLOR").is_some()
            || std::env::var("TERM").map(|t| t == "dumb").unwrap_or(false)
            || !std::io::stdout().is_terminal();
        let mode = if plain {
            ColorMode::Plain
        } else if std::env::var("COLORTERM")
            .map(|c| c.contains("truecolor") || c.contains("24bit"))
            .unwrap_or(false)
        {
            ColorMode::Truecolor
        } else {
            ColorMode::Ansi256
        };
        let unicode = ["LC_ALL", "LC_CTYPE", "LANG"]
            .iter()
            .find_map(|k| std::env::var(k).ok().filter(|v| !v.is_empty()))
            .map(|v| v.to_lowercase().contains("utf"))
            .unwrap_or(true); // modern default: assume UTF-8 when unstated
        Theme { mode, unicode }
    }

    /// Scale a palette color's brightness (breathing). 1.0 = rest.
    fn fg(&self, rgb: (u8, u8, u8), brightness: f32) -> String {
        match self.mode {
            ColorMode::Plain => String::new(),
            ColorMode::Truecolor => {
                let s = |c: u8| ((c as f32 * brightness).min(255.0)) as u8;
                format!("\x1b[38;2;{};{};{}m", s(rgb.0), s(rgb.1), s(rgb.2))
            }
            ColorMode::Ansi256 => {
                // Three-step breathing on the indexed ramp for the ice family;
                // other colors just switch on/off dim.
                let i = if rgb == ICE || rgb == ICE_DIM {
                    if brightness >= 1.15 {
                        159 // brightest ice
                    } else if brightness >= 0.9 {
                        idx256(rgb)
                    } else {
                        74
                    }
                } else {
                    idx256(rgb)
                };
                format!("\x1b[38;5;{i}m")
            }
        }
    }

    fn reset(&self) -> &'static str {
        if self.mode == ColorMode::Plain {
            ""
        } else {
            "\x1b[0m"
        }
    }

    fn bold(&self) -> &'static str {
        if self.mode == ColorMode::Plain {
            ""
        } else {
            "\x1b[1m"
        }
    }

    // ── public paint helpers (rest brightness) ────────────────────────────
    pub fn ice(&self, s: &str) -> String {
        format!("{}{s}{}", self.fg(ICE, 1.0), self.reset())
    }
    pub fn ice_bold(&self, s: &str) -> String {
        format!("{}{}{s}{}", self.bold(), self.fg(ICE, 1.0), self.reset())
    }
    pub fn dim(&self, s: &str) -> String {
        format!("{}{s}{}", self.fg(SLATE, 1.0), self.reset())
    }
    pub fn faint(&self, s: &str) -> String {
        format!("{}{s}{}", self.fg(FAINT, 1.0), self.reset())
    }
    pub fn amber(&self, s: &str) -> String {
        format!("{}{s}{}", self.fg(AMBER, 1.0), self.reset())
    }
    pub fn ok(&self, s: &str) -> String {
        format!("{}{s}{}", self.fg(GREEN, 1.0), self.reset())
    }

    /// The REPL prompt — a seed leaning into the wind.
    pub fn prompt(&self) -> String {
        if !self.unicode {
            return format!("{}{}> {}", self.bold(), self.fg(ICE, 1.0), self.reset());
        }
        format!("{}{}» {}", self.bold(), self.fg(ICE, 1.0), self.reset())
    }

    /// Accent color for the ratatui TUI (truecolor when the terminal has it).
    pub fn tui_accent(&self) -> (bool, (u8, u8, u8), u8) {
        (self.mode == ColorMode::Truecolor, ICE, idx256(ICE))
    }
}

// ── the dandelion ─────────────────────────────────────────────────────────

const BANNER_ROWS: usize = 6;
/// Loose seeds downwind of the head: (row, col-at-rest, glyph cycle).
/// Cols are absolute; the drift field wraps inside [SEED_MIN, SEED_MAX).
const SEEDS: &[(usize, usize, [char; 3])] = &[
    (0, 30, ['\u{b7}', '\u{2d9}', '\u{b7}']),    // · ˙ ·
    (0, 45, ['\u{2d9}', '\u{b7}', '\u{2d9}']),   // ˙ · ˙
    (1, 35, ['\u{2727}', '\u{b7}', '\u{2727}']), // ✧ · ✧  (twinkle)
    (1, 52, ['\u{b7}', '\u{b7}', '\u{2d9}']),
    (2, 27, ['\u{2d9}', '\u{2d9}', '\u{b7}']),
    (2, 40, ['\u{b7}', '\u{2d9}', '\u{b7}']),
    (2, 55, ['\u{22c6}', '\u{b7}', '\u{22c6}']), // ⋆ · ⋆
    (3, 32, ['\u{b7}', '\u{b7}', '\u{b7}']),
    (3, 47, ['\u{2d9}', '\u{b7}', '\u{2d9}']),
];
const SEED_MIN: usize = 24;
const SEED_MAX: usize = 62;

/// One rendered frame of the banner. `breath` in [0.0, 1.0] — 0 exhaled,
/// 1 fully inhaled; drives ray length + brightness. `drift` shifts the loose
/// seeds downwind. Rows are padded so redraws fully overwrite.
fn frame(t: &Theme, breath: f32, drift: usize, tagline: &str) -> Vec<String> {
    let b = (breath * 2.0).round() as usize; // 0 | 1 | 2 — ray segments
    let glow = 0.82 + 0.38 * breath; // brightness sweep
    let rays = "\u{2014}".repeat(1 + b); // — —— ———
    let pad = " ".repeat(2usize.saturating_sub(b));

    // The head: five rows sharing one vertical axis — ✦, |, ❁, |, ' and the
    // stem all sit on column 10, so nothing wobbles as the breath resizes.
    let head: [String; 5] = [
        "       .  \u{2726}  .".to_string(),       //    .  ✦  .
        "     \u{b7}  \\ | /  \u{b7}".to_string(), //  ·  \ | /  ·
        format!("    {pad}\u{b7} {rays} \u{2741} {rays} \u{b7}"), // · —— ❁ —— ·
        "     \u{b7}  / | \\  \u{b7}".to_string(), //  ·  / | \  ·
        "       '  |  '".to_string(),
    ];

    let mut rows: Vec<String> = Vec::with_capacity(BANNER_ROWS);
    for (i, base) in head.iter().enumerate() {
        let mut cells: Vec<char> = base.chars().collect();
        cells.resize(SEED_MAX + 2, ' ');
        // Overlay drifting seeds (rows 0..4 share the field).
        let mut line = String::new();
        let head_bright = t.fg(ICE, glow);
        let ray_col = t.fg(ICE_DIM, glow);
        // Paint the head portion.
        let head_len = base.chars().count();
        for (ci, ch) in cells.iter().enumerate().take(head_len) {
            match ch {
                '\u{2741}' | '\u{2726}' => {
                    line.push_str(&head_bright);
                    line.push(*ch);
                }
                '\u{2014}' | '\\' | '/' | '|' | '\'' | '.' => {
                    line.push_str(&ray_col);
                    line.push(*ch);
                }
                '\u{b7}' => {
                    line.push_str(&t.fg(SLATE, glow));
                    line.push(*ch);
                }
                _ => {
                    let _ = ci;
                    line.push(*ch);
                }
            }
        }
        line.push_str(t.reset());
        // Paint the seed field beyond the head.
        let mut field: Vec<(usize, char)> = Vec::new();
        for (row, col0, glyphs) in SEEDS {
            if *row == i {
                let span = SEED_MAX - SEED_MIN;
                let col = SEED_MIN + ((*col0 - SEED_MIN) + drift) % span;
                let glyph = glyphs[(drift / 2) % 3];
                field.push((col, glyph));
            }
        }
        field.sort();
        let mut cursor = head_len.max(SEED_MIN.min(SEED_MAX));
        for (col, glyph) in field {
            if col <= cursor {
                continue;
            }
            line.push_str(&" ".repeat(col - cursor));
            // Far seeds fade — distance is the wind carrying them off.
            let far = col > (SEED_MIN + (SEED_MAX - SEED_MIN) * 2 / 3);
            let color = if far {
                t.fg(FAINT, 1.0)
            } else {
                t.fg(SLATE, glow)
            };
            line.push_str(&color);
            line.push(glyph);
            line.push_str(t.reset());
            cursor = col + 1;
        }
        // Pad to full width so animation redraws leave no ghosts.
        if cursor < SEED_MAX + 2 {
            line.push_str(&" ".repeat(SEED_MAX + 2 - cursor));
        }
        rows.push(line);
    }

    // The stem + wordmark row.
    let mark = format!("{}{}r o k h a{}", t.bold(), t.fg(ICE, glow), t.reset());
    let mut last = format!(
        "          {}|{}     {mark}  {}\u{b7}  {tagline}{}",
        t.fg(ICE_DIM, glow),
        t.reset(),
        t.fg(FAINT, 1.0),
        t.reset()
    );
    // Pad the visible tail (ANSI-invisible bytes don't matter for overwrite,
    // but trailing spaces do).
    last.push_str("          ");
    rows.push(last);
    rows
}

fn frame_ascii(tagline: &str) -> Vec<String> {
    vec![
        "       .  *  .".into(),
        "     .  \\ | /  .".into(),
        "     . -- * -- .        .       .".into(),
        "     .  / | \\  .    .        .".into(),
        "       '  |  '".into(),
        format!("          |     r o k h a  .  {tagline}"),
    ]
}

/// Print the launch banner. Animates one breath cycle (~1s) on interactive
/// color terminals; prints a single static frame everywhere else.
pub fn banner(tagline: &str) {
    let t = Theme::detect();
    println!();
    if !t.unicode {
        for row in frame_ascii(tagline) {
            println!("{row}");
        }
        println!();
        return;
    }

    let animate = t.mode != ColorMode::Plain
        && std::env::var_os("RO_NO_ANIM").is_none()
        && std::io::stdout().is_terminal();

    if !animate {
        for row in frame(&t, 0.5, 0, tagline) {
            println!("{row}");
        }
        println!();
        return;
    }

    // One breath: inhale → hold → exhale → rest, seeds drifting throughout.
    // 14 frames × 70ms ≈ 1s, then the resting frame stays on screen.
    const BREATH: [f32; 14] = [
        0.15, 0.3, 0.5, 0.7, 0.88, 1.0, 1.0, 0.88, 0.7, 0.5, 0.35, 0.25, 0.4, 0.5,
    ];
    let mut out = std::io::stdout();
    let _ = write!(out, "\x1b[?25l"); // hide cursor
    for (i, breath) in BREATH.iter().enumerate() {
        if i > 0 {
            let _ = write!(out, "\x1b[{BANNER_ROWS}A");
        }
        for row in frame(&t, *breath, i, tagline) {
            let _ = writeln!(out, "\r{row}");
        }
        let _ = out.flush();
        std::thread::sleep(std::time::Duration::from_millis(70));
    }
    let _ = write!(out, "\x1b[?25h"); // show cursor
    let _ = out.flush();
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn plain() -> Theme {
        Theme {
            mode: ColorMode::Plain,
            unicode: true,
        }
    }

    /// Char column of the axis glyph on one row (None = row has no axis glyph).
    fn axis(row: &str, glyph: char) -> usize {
        row.chars().position(|c| c == glyph).unwrap()
    }

    #[test]
    fn frames_hold_shape_across_the_breath() {
        // Every frame has the same row count and every axis glyph — crown,
        // rays' center, head, base, stem — sits on column 10, at every breath
        // size. Redraw-in-place and the vertical alignment both depend on it.
        for (i, b) in [0.0f32, 0.25, 0.5, 0.75, 1.0].iter().enumerate() {
            let rows = frame(&plain(), *b, i * 3, "the local agent");
            assert_eq!(rows.len(), BANNER_ROWS);
            assert!(rows[5].contains("r o k h a"));
            assert_eq!(axis(&rows[0], '\u{2726}'), 10, "crown off-axis at {b}");
            assert_eq!(axis(&rows[1], '|'), 10, "upper rays off-axis at {b}");
            assert_eq!(axis(&rows[2], '\u{2741}'), 10, "head off-axis at {b}");
            assert_eq!(axis(&rows[3], '|'), 10, "lower rays off-axis at {b}");
            assert_eq!(axis(&rows[4], '|'), 10, "base off-axis at {b}");
            assert_eq!(axis(&rows[5], '|'), 10, "stem off-axis at {b}");
        }
    }

    #[test]
    fn ascii_fallback_shares_the_axis() {
        let rows = frame_ascii("the local agent");
        assert_eq!(axis(&rows[0], '*'), 10);
        assert_eq!(axis(&rows[1], '|'), 10);
        assert_eq!(axis(&rows[2], '*'), 10);
        assert_eq!(axis(&rows[3], '|'), 10);
        assert_eq!(axis(&rows[4], '|'), 10);
        assert_eq!(axis(&rows[5], '|'), 10);
    }

    #[test]
    fn seeds_drift_downwind_and_wrap() {
        let a = frame(&plain(), 0.5, 0, "x").join("\n");
        let b = frame(&plain(), 0.5, 5, "x").join("\n");
        assert_ne!(a, b, "drift produced no movement");
        // No seed ever lands beyond the field (ghost columns break redraw).
        for row in frame(&plain(), 0.5, 37, "x") {
            assert!(row.chars().count() <= SEED_MAX + 60); // ansi-free in plain
        }
    }

    #[test]
    fn plain_mode_emits_no_escape_codes() {
        let rows = frame(&plain(), 1.0, 3, "the local agent");
        assert!(!rows.join("").contains('\x1b'));
        let t = plain();
        assert!(!t.ice("x").contains('\x1b'));
        assert!(!t.prompt().contains('\x1b'));
    }

    #[test]
    fn ascii_fallback_is_pure_ascii() {
        for row in frame_ascii("the local agent") {
            assert!(row.is_ascii(), "non-ascii in fallback: {row}");
        }
    }
}
