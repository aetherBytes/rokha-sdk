mod api_client;
mod cli;
mod config;
mod credentials;
mod mcp;
mod tui;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "ro",
    bin_name = "ro",
    version,
    about = "Rokha CLI — thin client for rokha.ai"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Print CLI + schema version
    Version,
    /// Show login state, remote reachability, and schema match
    Status,
    /// Alias for `status` — kept for muscle memory
    Health,
    /// Log in to your Rokha account via browser (RFC 8628 device flow)
    Login,
    /// Show the current logged-in identity
    Whoami,
    /// Remove local credentials
    Logout,
    /// Show or modify CLI config (base URL, etc.)
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
    /// Browse the Rokha Registry
    Tools {
        #[command(subcommand)]
        action: ToolsAction,
    },
    /// Send a one-shot message to the Rokha agent
    Chat { message: String },
    /// Launch the TUI dashboard
    Tui,
    /// MCP server (JSON-RPC over stdio)
    Mcp {
        #[command(subcommand)]
        action: McpAction,
    },
}

#[derive(Subcommand)]
enum ToolsAction {
    List,
    Info { name: String },
}

#[derive(Subcommand)]
enum McpAction {
    Serve,
}

#[derive(Subcommand)]
enum ConfigAction {
    /// Print current config
    Show,
    /// Set the remote Rokha base URL (default: https://api.rokha.ai)
    SetBaseUrl { url: String },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let cfg = config::Config::load();
    let client = api_client::RokhaClient::new(&cfg.erebus_url);

    let exit_code: i32 = match cli.command {
        Commands::Version => {
            cli::status::version().await;
            0
        }
        Commands::Status | Commands::Health => {
            cli::status::run(&client).await;
            0
        }
        Commands::Tools { action } => {
            match action {
                ToolsAction::List => cli::tools::list(&client).await,
                ToolsAction::Info { name } => cli::tools::info(&client, &name).await,
            }
            0
        }
        Commands::Chat { message } => cli::agents::chat(&client, &message).await,
        Commands::Login => cli::auth::login(client.base_url()).await,
        Commands::Whoami => cli::auth::whoami().await,
        Commands::Logout => cli::auth::logout().await,
        Commands::Config { action } => match action {
            ConfigAction::Show => cli::config_cmd::show(&cfg),
            ConfigAction::SetBaseUrl { url } => cli::config_cmd::set_base_url(&url),
        },
        Commands::Tui => {
            tui::dashboard::run(&client).await;
            0
        }
        Commands::Mcp { action } => {
            match action {
                McpAction::Serve => mcp::server::serve(&client).await,
            }
            0
        }
    };

    if exit_code != 0 {
        std::process::exit(exit_code);
    }
}
