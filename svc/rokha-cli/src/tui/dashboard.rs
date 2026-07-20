use crate::api_client::{Listing, RokhaClient};
use crate::credentials;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, Tabs},
};
use std::io;

/// A snapshot of live platform + local state, fetched once per refresh. The
/// TUI is a read-only heads-up monitor for the local agent: it shows what the
/// platform reports, it does not fake anything.
struct Dash {
    base_url: String,
    schema_client: String,
    schema_server: Option<String>,
    reachable: bool,
    login: Option<String>,
    listings: Vec<Listing>,
    listings_err: Option<String>,
    agent: Option<serde_json::Value>,
}

enum Tab {
    Status,
    Registry,
    Agent,
}

impl Tab {
    fn index(&self) -> usize {
        match self {
            Tab::Status => 0,
            Tab::Registry => 1,
            Tab::Agent => 2,
        }
    }
    fn titles() -> Vec<&'static str> {
        vec!["[1] Status", "[2] Registry", "[3] Agent"]
    }
}

enum Action {
    Quit,
    Refresh,
}

async fn fetch(client: &RokhaClient) -> Dash {
    let creds = credentials::load();
    let login = creds.as_ref().map(|c| {
        format!(
            "{} ({}, {} tier)",
            c.identity.identity, c.identity.auth_method, c.identity.tier
        )
    });

    let schema = client.schema_version().await;
    let reachable = schema.is_ok();
    let schema_server = schema.ok().map(|s| s.version);

    let (listings, listings_err) = match client.list_tools().await {
        Ok(v) => (v, None),
        Err(e) => (Vec::new(), Some(e.to_string())),
    };

    let agent = client.agent_status().await.ok();

    Dash {
        base_url: client.base_url().to_string(),
        schema_client: crate::api_client::SCHEMA_VERSION.to_string(),
        schema_server,
        reachable,
        login,
        listings,
        listings_err,
        agent,
    }
}

pub async fn run(client: &RokhaClient) {
    loop {
        let data = fetch(client).await;
        match run_ui(&data) {
            Ok(Action::Quit) => break,
            Ok(Action::Refresh) => continue,
            Err(e) => {
                eprintln!("TUI error: {e}");
                break;
            }
        }
    }
}

fn run_ui(data: &Dash) -> io::Result<Action> {
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    let mut active_tab = Tab::Status;

    let action = loop {
        terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(0),
                    Constraint::Length(1),
                ])
                .split(frame.area());

            let tabs = Tabs::new(Tab::titles())
                .block(Block::default().borders(Borders::ALL).title(" Rokha "))
                .select(active_tab.index())
                .highlight_style(
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                );
            frame.render_widget(tabs, chunks[0]);

            let body = match active_tab {
                Tab::Status => Paragraph::new(status_lines(data))
                    .block(Block::default().borders(Borders::ALL).title(" Status ")),
                Tab::Registry => Paragraph::new(registry_lines(data))
                    .block(Block::default().borders(Borders::ALL).title(" Registry ")),
                Tab::Agent => Paragraph::new(agent_lines(data))
                    .block(Block::default().borders(Borders::ALL).title(" Agent ")),
            };
            frame.render_widget(body, chunks[1]);

            let status = Paragraph::new(format!(
                " q: quit | r: refresh | 1/2/3: tabs | {} ",
                data.base_url
            ))
            .style(Style::default().fg(Color::DarkGray));
            frame.render_widget(status, chunks[2]);
        })?;

        if event::poll(std::time::Duration::from_millis(150))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break Action::Quit,
                    KeyCode::Char('r') => break Action::Refresh,
                    KeyCode::Char('1') => active_tab = Tab::Status,
                    KeyCode::Char('2') => active_tab = Tab::Registry,
                    KeyCode::Char('3') => active_tab = Tab::Agent,
                    _ => {}
                }
            }
        }
    };

    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;
    Ok(action)
}

fn status_lines(d: &Dash) -> String {
    let mut out = String::new();
    out.push_str(&format!("Remote:   {}\n", d.base_url));
    out.push_str(&format!(
        "Reach:    {}\n",
        if d.reachable { "online" } else { "UNREACHABLE" }
    ));
    match &d.schema_server {
        Some(s) if *s == d.schema_client => out.push_str(&format!("Schema:   {s} (matches CLI)\n")),
        Some(s) => out.push_str(&format!(
            "Schema:   server {s} / CLI {} — DRIFT\n",
            d.schema_client
        )),
        None => out.push_str(&format!(
            "Schema:   CLI {} (server unknown)\n",
            d.schema_client
        )),
    }
    match &d.login {
        Some(id) => out.push_str(&format!("Login:    {id}\n")),
        None => out.push_str("Login:    anonymous — run `ro login`\n"),
    }
    out.push_str("\nThe local agent is a thin interface; runs, search, and\ntools all execute on the platform. Press r to refresh.");
    out
}

fn registry_lines(d: &Dash) -> String {
    if let Some(err) = &d.listings_err {
        return format!("Could not load registry: {err}");
    }
    if d.listings.is_empty() {
        return "No listings returned.".to_string();
    }
    let mut out = format!("{} listing(s) — showing first 20:\n\n", d.listings.len());
    for t in d.listings.iter().take(20) {
        out.push_str(&format!(
            "• {:<30} {:<10} {}\n",
            t.name(),
            t.class(),
            t.provider()
        ));
    }
    out
}

fn agent_lines(d: &Dash) -> String {
    match &d.agent {
        None => "Agent status unavailable (offline or auth required).".to_string(),
        Some(v) => {
            let get = |k: &str| {
                v.get(k)
                    .map(|x| match x {
                        serde_json::Value::String(s) => s.clone(),
                        other => other.to_string(),
                    })
                    .unwrap_or_else(|| "—".to_string())
            };
            format!(
                "Status:   {}\nModel:    {}\nHealthy:  {}\n\nRaw:\n{}",
                get("status"),
                get("model"),
                get("healthy"),
                serde_json::to_string_pretty(v).unwrap_or_default()
            )
        }
    }
}
