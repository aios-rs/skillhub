mod application;
mod domain;
mod infrastructure;
mod tui;

use application::service::SkillHubService;
use clap::{Parser, Subcommand};
use infrastructure::config::{self, Config};
use infrastructure::client::SkillHubClient;
use infrastructure::repository::{auth_repository_impl::AuthRepositoryImpl, skill_repository_impl::SkillRepositoryImpl};
use std::sync::Arc;

#[derive(Parser)]
#[command(name = "skillhub", version, about = "TUI for SkillHub - AI Agent Skill Registry")]
struct Cli {
    /// Registry URL (overrides config)
    #[arg(long = "registry", short = 'r', global = true)]
    registry: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize skillhub configuration
    Init,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Init) => run_init().await,
        None => run_tui(cli.registry).await,
    }
}

async fn run_init() -> Result<(), Box<dyn std::error::Error>> {
    use colored::Colorize;
    use dialoguer::{Input, Select};

    println!("{}", "╔══════════════════════════════════════╗".cyan());
    println!("{}", "║   SkillHub Configuration Wizard      ║".cyan());
    println!("{}", "╚══════════════════════════════════════╝".cyan());
    println!();

    // Registry URL
    let default_url = "http://localhost:3001";
    let registry_url: String = Input::new()
        .with_prompt("Registry URL")
        .default(default_url.to_string())
        .interact_text()?;

    // Auth method
    let auth_options = vec![
        "Skip (use without login)",
        "Username / Password",
        "App ID / App Secret",
    ];
    let auth_choice = Select::new()
        .with_prompt("Authentication method")
        .items(&auth_options)
        .default(0)
        .interact()?;

    let mut config = Config::default();
    config.registry.url = registry_url;

    match auth_choice {
        1 => {
            let username: String = Input::new()
                .with_prompt("Username")
                .interact_text()?;
            let password: String = Input::new()
                .with_prompt("Password")
                .interact_text()?;

            // Try login to get token
            let client = SkillHubClient::new(config.registry.url.clone(), None);
            match client.login(&username, &password).await {
                Ok(token) => {
                    config.auth.token = Some(token);
                    println!("\n{}", "✓ Login successful!".green());
                }
                Err(e) => {
                    println!("\n{} {}", "✗ Login failed:".red(), e);
                    println!("{}", "  Config saved without token. You can login later in TUI.".yellow());
                }
            }
        }
        2 => {
            let app_id: String = Input::new()
                .with_prompt("App ID")
                .interact_text()?;
            let app_secret: String = Input::new()
                .with_prompt("App Secret")
                .interact_text()?;

            // Try app login to get token
            let client = SkillHubClient::new(config.registry.url.clone(), None);
            match client.login_with_app(&app_id, &app_secret).await {
                Ok(token) => {
                    config.auth.token = Some(token);
                    config.auth.app_id = Some(app_id);
                    config.auth.app_secret = Some(app_secret);
                    println!("\n{}", "✓ App authentication successful!".green());
                }
                Err(e) => {
                    println!("\n{} {}", "✗ App authentication failed:".red(), e);
                    println!("{}", "  Config saved without token. You can login later in TUI.".yellow());
                }
            }
        }
        _ => {
            println!("\n{}", "ℹ Skipping authentication.".dimmed());
        }
    }

    config::save(&config)?;

    let config_path = dirs::home_dir()
        .map(|h| h.join(".skillhub").join("config.toml"))
        .map(|p| p.display().to_string())
        .unwrap_or_default();

    println!("\n{} {}", "✓ Config saved to".green(), config_path.bold());
    println!("{}", "  Run `skillhub` to start the TUI.".dimmed());

    Ok(())
}

async fn run_tui(registry_override: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = config::load()?;

    // CLI --registry overrides config
    if let Some(url) = registry_override {
        config.registry.url = url;
    }

    let has_app_credentials = config.auth.has_app_credentials();
    let registry_url = config.registry.url.clone();

    // Create client (may start without token)
    let client = Arc::new(SkillHubClient::new(
        registry_url.clone(),
        config.auth.token.clone(),
    ));

    // If app credentials exist but no token, try auto-login
    if has_app_credentials && config.auth.token.is_none() {
        if let (Some(app_id), Some(app_secret)) = (&config.auth.app_id, &config.auth.app_secret) {
            match client.login_with_app(app_id, app_secret).await {
                Ok(token) => {
                    client.set_token(token.clone());
                    config.auth.token = Some(token);
                }
                Err(_) => {}
            }
        }
    }

    // Create repositories
    let auth_repo = Arc::new(AuthRepositoryImpl::new(client.clone()))
        as Arc<dyn domain::repository::auth_repository::AuthRepository>;
    let skill_repo = Arc::new(SkillRepositoryImpl::new(client.clone()))
        as Arc<dyn domain::repository::skill_repository::SkillRepository>;

    let service = Arc::new(
        SkillHubService::new(skill_repo)
            .with_auth_repo(auth_repo)
    );

    let is_authenticated = client.has_token();

    // Run TUI (always starts on home, login is lazy)
    let _should_save_config = tui::runner::run(
        service,
        client,
        is_authenticated,
        registry_url,
    ).await?;

    Ok(())
}
