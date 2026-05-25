use crate::api_client::RokhaClient;
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

enum Tab {
    Chat,
    Crossroads,
    Rokha,
}

impl Tab {
    fn index(&self) -> usize {
        match self {
            Tab::Chat => 0,
            Tab::Crossroads => 1,
            Tab::Rokha => 2,
        }
    }

    fn titles() -> Vec<&'static str> {
        vec!["[1] Chat", "[2] Rokha Registry", "[3] Rokha"]
    }
}

pub async fn run(_client: &RokhaClient) {
    if let Err(e) = run_tui() {
        eprintln!("TUI error: {}", e);
    }
}

fn run_tui() -> io::Result<()> {
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;

    let mut active_tab = Tab::Chat;

    loop {
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
                .highlight_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
            frame.render_widget(tabs, chunks[0]);

            let content = match active_tab {
                Tab::Chat => Paragraph::new("Rokha Agent Chat\n\nType a message to begin.")
                    .block(Block::default().borders(Borders::ALL).title(" Chat ")),
                Tab::Crossroads => {
                    Paragraph::new("Rokha Registry\n\nBrowse tools and COWs.")
                        .block(Block::default().borders(Borders::ALL).title(" Rokha Registry "))
                }
                Tab::Rokha => {
                    Paragraph::new("Rokha Editor\n\nBreathe. Flow. Create.")
                        .block(Block::default().borders(Borders::ALL).title(" Rokha "))
                }
            };
            frame.render_widget(content, chunks[1]);

            let status = Paragraph::new(" q: quit | 1/2/3: switch tabs | erebus: localhost:3000")
                .style(Style::default().fg(Color::DarkGray));
            frame.render_widget(status, chunks[2]);
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('1') => active_tab = Tab::Chat,
                    KeyCode::Char('2') => active_tab = Tab::Crossroads,
                    KeyCode::Char('3') => active_tab = Tab::Rokha,
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
