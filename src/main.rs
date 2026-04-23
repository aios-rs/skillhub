mod application;
mod domain;
mod infrastructure;
mod tui;

use application::service::SkillHubService;
use clap::{Parser, Subcommand};
use colored::Colorize;
use infrastructure::client::SkillHubClient;
use infrastructure::config::{self, Config};
use infrastructure::repository::{auth_repository_impl::AuthRepositoryImpl, skill_repository_impl::SkillRepositoryImpl};
use std::sync::Arc;

#[derive(Parser)]
#[command(name = "skillhub", version, about = "SkillHub - AI Agent Skill Registry CLI")]
struct Cli {
    /// Registry URL (overrides config)
    #[arg(long = "registry", short = 'r', global = true)]
    registry: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Launch the interactive TUI (default when no command given)
    Tui,

    /// Initialize skillhub configuration
    Init,

    /// Login and save credentials
    Login {
        /// Username
        username: String,
        /// Password
        password: String,
    },

    /// Login with app credentials
    LoginApp {
        /// App ID
        app_id: String,
        /// App Secret
        app_secret: String,
    },

    /// Search skills
    Search {
        /// Search query
        query: Option<String>,
        /// Filter by namespace
        #[arg(long, short = 'n')]
        namespace: Option<String>,
        /// Filter by labels (can be repeated)
        #[arg(long, short = 'l')]
        labels: Vec<String>,
        /// Sort order (newest, downloads, rating, name)
        #[arg(long, short = 's', default_value = "newest")]
        sort: String,
        /// Page number (1-based)
        #[arg(long, default_value_t = 1)]
        page: u32,
        /// Page size
        #[arg(long, default_value_t = 20)]
        page_size: u32,
    },

    /// Show skill detail
    Info {
        /// Namespace slug
        namespace: String,
        /// Skill slug
        slug: String,
    },

    /// List skill versions
    Versions {
        /// Namespace slug
        namespace: String,
        /// Skill slug
        slug: String,
        /// Page number (1-based)
        #[arg(long, default_value_t = 1)]
        page: u32,
        /// Page size
        #[arg(long, default_value_t = 20)]
        page_size: u32,
    },

    /// Download a skill bundle
    Download {
        /// Namespace slug
        namespace: String,
        /// Skill slug
        slug: String,
        /// Version (defaults to latest)
        version: Option<String>,
        /// Output file path (defaults to stdout)
        #[arg(long, short = 'o')]
        output: Option<String>,
    },

    /// Publish a skill
    Publish {
        /// Namespace slug
        namespace: String,
        /// Path to skill archive file (.zip)
        file: String,
        /// Visibility (public, private, hidden)
        #[arg(long, short = 'v', default_value = "public")]
        visibility: String,
    },

    /// Star a skill
    Star {
        /// Skill ID
        skill_id: String,
    },

    /// Unstar a skill
    Unstar {
        /// Skill ID
        skill_id: String,
    },

    /// Rate a skill (1-5)
    Rate {
        /// Skill ID
        skill_id: String,
        /// Rating score (1-5)
        score: i16,
    },

    /// Show hub statistics
    Stats,

    /// List namespaces
    Namespaces,

    /// List labels
    Labels,

    /// List your skills
    MySkills {
        /// Page number (1-based)
        #[arg(long, default_value_t = 1)]
        page: u32,
        /// Page size
        #[arg(long, default_value_t = 20)]
        page_size: u32,
    },

    /// List your starred skills
    MyStars {
        /// Page number (1-based)
        #[arg(long, default_value_t = 1)]
        page: u32,
        /// Page size
        #[arg(long, default_value_t = 20)]
        page_size: u32,
    },

    /// Show your profile
    Profile,

    /// List notifications
    Notifications {
        /// Page number (1-based)
        #[arg(long, default_value_t = 1)]
        page: u32,
        /// Page size
        #[arg(long, default_value_t = 20)]
        page_size: u32,
    },

    /// Mark a notification as read
    ReadNotification {
        /// Notification ID
        id: String,
    },

    /// Mark all notifications as read
    ReadAllNotifications,

    /// List API tokens
    Tokens {
        /// Page number (1-based)
        #[arg(long, default_value_t = 1)]
        page: u32,
        /// Page size
        #[arg(long, default_value_t = 20)]
        page_size: u32,
    },

    /// Delete an API token
    DeleteToken {
        /// Token ID
        id: String,
    },

    // ===== 新增指令 =====

    /// Verify your authentication status
    Whoami,

    /// Browse latest skills
    Explore {
        /// Maximum number of results (1-200)
        #[arg(long, default_value_t = 25)]
        limit: u32,
        /// Sort order (newest, downloads, rating, name)
        #[arg(long, short = 's', default_value = "newest")]
        sort: String,
    },

    /// Inspect a skill in detail
    Inspect {
        /// Namespace slug
        namespace: String,
        /// Skill slug
        slug: String,
        /// List version history
        #[arg(long)]
        versions: bool,
        /// List files in the specified version
        #[arg(long)]
        files: bool,
        /// Show content of a specific file
        #[arg(long, value_name = "PATH")]
        file: Option<String>,
        /// Version to inspect (default: latest)
        #[arg(long)]
        version: Option<String>,
    },

    /// Report a skill
    Report {
        /// Namespace slug
        namespace: String,
        /// Skill slug
        slug: String,
        /// Report reason
        reason: String,
        /// Detailed description
        #[arg(long, short = 'd')]
        description: Option<String>,
    },

    /// Install a skill locally
    Install {
        /// Namespace slug
        namespace: String,
        /// Skill slug
        slug: String,
        /// Version to install (default: latest)
        #[arg(long)]
        version: Option<String>,
        /// Skip confirmation prompt
        #[arg(long, short = 'y')]
        yes: bool,
    },

    /// Uninstall a local skill
    Uninstall {
        /// Namespace slug
        namespace: String,
        /// Skill slug
        slug: String,
        /// Skip confirmation prompt
        #[arg(long, short = 'y')]
        yes: bool,
    },

    /// List installed skills
    List,

    /// Update installed skills
    Update {
        /// Skill to update (format: namespace/slug)
        skill: Option<String>,
        /// Update all installed skills
        #[arg(long)]
        all: bool,
    },

    /// Skill lifecycle management commands
    #[command(subcommand)]
    Skill(SkillCommands),

    /// Review management commands
    #[command(subcommand)]
    Review(ReviewCommands),

    /// Transfer ownership commands
    #[command(subcommand)]
    Transfer(TransferCommands),

    /// Admin commands
    #[command(subcommand)]
    Admin(AdminCommands),
}

#[derive(Subcommand)]
enum SkillCommands {
    /// Delete a skill (soft-delete/archive)
    Delete {
        /// Namespace slug
        namespace: String,
        /// Skill slug
        slug: String,
    },
    /// Restore a deleted skill
    Undelete {
        /// Namespace slug
        namespace: String,
        /// Skill slug
        slug: String,
    },
    /// Hide a skill (alias for delete)
    Hide {
        /// Namespace slug
        namespace: String,
        /// Skill slug
        slug: String,
    },
    /// Unhide a skill (alias for undelete)
    Unhide {
        /// Namespace slug
        namespace: String,
        /// Skill slug
        slug: String,
    },
    /// Rename a skill
    Rename {
        /// Namespace slug
        namespace: String,
        /// Current skill slug
        slug: String,
        /// New slug
        new_slug: String,
    },
    /// Yank a version
    Yank {
        /// Namespace slug
        namespace: String,
        /// Skill slug
        slug: String,
        /// Version string
        version: String,
        /// Reason for yanking
        #[arg(short, long)]
        reason: Option<String>,
    },
    /// Re-release a yanked version
    Rerelease {
        /// Namespace slug
        namespace: String,
        /// Skill slug
        slug: String,
        /// Version string
        version: String,
    },
}

#[derive(Subcommand)]
enum ReviewCommands {
    /// Submit a skill for review
    Submit {
        /// Namespace slug
        namespace: String,
        /// Skill slug
        slug: String,
        /// Version string
        version: String,
        /// Review comment
        #[arg(short, long)]
        comment: Option<String>,
    },
    /// Withdraw a review
    Withdraw {
        /// Namespace slug
        namespace: String,
        /// Skill slug
        slug: String,
        /// Version string
        version: String,
    },
    /// List review tasks
    List {
        /// Filter by status
        #[arg(short, long)]
        status: Option<String>,
    },
}

#[derive(Subcommand)]
enum TransferCommands {
    /// Request ownership transfer
    Request {
        /// Namespace slug
        namespace: String,
        /// Skill slug
        slug: String,
        /// Target user handle
        target_handle: String,
        /// Transfer message
        #[arg(short, long)]
        message: Option<String>,
    },
    /// List transfer requests
    List {
        /// Show outgoing instead of incoming
        #[arg(short, long)]
        outgoing: bool,
    },
    /// Accept a transfer request
    Accept {
        /// Transfer request ID
        id: String,
    },
    /// Reject a transfer request
    Reject {
        /// Transfer request ID
        id: String,
    },
    /// Cancel a transfer request
    Cancel {
        /// Transfer request ID
        id: String,
    },
}

#[derive(Subcommand)]
enum AdminCommands {
    /// Ban a user
    BanUser {
        /// User handle or ID
        handle: String,
        /// Ban reason
        #[arg(short, long)]
        reason: Option<String>,
        /// Treat argument as user ID
        #[arg(short, long)]
        id: bool,
    },
    /// Change user role
    SetRole {
        /// User handle or ID
        handle: String,
        /// New role
        role: String,
        /// Treat argument as user ID
        #[arg(short, long)]
        id: bool,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Init) => run_init().await,
        Some(Commands::Tui) | None => run_tui(cli.registry).await,
        Some(cmd) => run_cli_command(cmd, cli.registry).await,
    }
}

async fn create_service(registry_override: Option<String>) -> Result<(Arc<SkillHubService>, Config), Box<dyn std::error::Error>> {
    let mut config = config::load()?;
    if let Some(url) = registry_override {
        config.registry.url = url;
    }

    let client = Arc::new(SkillHubClient::new(
        config.registry.url.clone(),
        config.auth.token.clone(),
    ));

    // Auto-login with app credentials
    if config.auth.has_app_credentials() && config.auth.token.is_none() {
        if let (Some(app_id), Some(app_secret)) = (&config.auth.app_id, &config.auth.app_secret) {
            match client.login_with_app(app_id, app_secret).await {
                Ok(tokens) => {
                    client.set_tokens(tokens.access_token.clone(), tokens.refresh_token.clone());
                    config.auth.token = Some(tokens.access_token);
                    config.auth.refresh_token = tokens.refresh_token;
                }
                Err(_) => {}
            }
        }
    }

    let auth_repo = Arc::new(AuthRepositoryImpl::new(client.clone()))
        as Arc<dyn domain::repository::auth_repository::AuthRepository>;
    let skill_repo = Arc::new(SkillRepositoryImpl::new(client.clone()))
        as Arc<dyn domain::repository::skill_repository::SkillRepository>;

    let service = Arc::new(
        SkillHubService::new(skill_repo).with_auth_repo(auth_repo),
    );

    Ok((service, config))
}

async fn run_cli_command(cmd: Commands, registry: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        Commands::Login { username, password } => {
            let (service, mut config) = create_service(registry).await?;
            match service.login(&username, &password).await {
                Ok(tokens) => {
                    config.auth.token = Some(tokens.access_token);
                    config.auth.refresh_token = tokens.refresh_token;
                    config::save(&config)?;
                    println!("{}", "Login successful! Token saved.".green());
                }
                Err(e) => eprintln!("{} {}", "Login failed:".red(), e),
            }
        }

        Commands::LoginApp { app_id, app_secret } => {
            let (service, mut config) = create_service(registry).await?;
            match service.login_with_app(&app_id, &app_secret).await {
                Ok(tokens) => {
                    config.auth.token = Some(tokens.access_token);
                    config.auth.refresh_token = tokens.refresh_token;
                    config.auth.app_id = Some(app_id);
                    config.auth.app_secret = Some(app_secret);
                    config::save(&config)?;
                    println!("{}", "App login successful! Token saved.".green());
                }
                Err(e) => eprintln!("{} {}", "App login failed:".red(), e),
            }
        }

        Commands::Search { query, namespace, labels, sort, page, page_size } => {
            let (service, _) = create_service(registry).await?;
            let (skills, total) = service.search_skills(query, namespace, labels, sort, page, page_size).await?;
            println!("{}", format!("Found {} skills (page {}):", total, page).cyan());
            for skill in &skills {
                let star = if skill.star_count > 0 { format!(" \u{2605}{}", skill.star_count) } else { String::new() };
                let rating = if skill.rating_count > 0 {
                    format!(" \u{2605}{:.1}({})", skill.rating_avg, skill.rating_count)
                } else {
                    String::new()
                };
                let version = skill.latest_version.as_deref().unwrap_or("-");
                println!(
                    "  {}/{} v{} [{}]{}{}  {}",
                    skill.namespace_slug,
                    skill.slug,
                    version,
                    skill.status,
                    star,
                    rating,
                    skill.summary.as_deref().unwrap_or("")
                );
            }
            if skills.is_empty() {
                println!("  No skills found.");
            }
        }

        Commands::Info { namespace, slug } => {
            let (service, _) = create_service(registry).await?;
            match service.get_skill_detail(&namespace, &slug).await? {
                Some(skill) => {
                    println!("{}", format!("{}: {}", skill.namespace_slug, skill.slug).cyan().bold());
                    if let Some(name) = &skill.display_name {
                        println!("  Name:     {}", name);
                    }
                    if let Some(summary) = &skill.summary {
                        println!("  Summary:  {}", summary);
                    }
                    println!("  Owner:    {}", skill.owner_name);
                    println!("  Status:   {}", skill.status);
                    println!("  Version:  {}", skill.latest_version.as_deref().unwrap_or("none"));
                    println!("  Downloads: {}", skill.download_count);
                    println!("  Stars:    {}", skill.star_count);
                    if skill.rating_count > 0 {
                        println!("  Rating:   {:.1}/5 ({})", skill.rating_avg, skill.rating_count);
                    }
                    if !skill.tags.is_empty() {
                        println!("  Tags:     {}", skill.tags.join(", "));
                    }
                    if !skill.labels.is_empty() {
                        let labels: Vec<&str> = skill.labels.iter().map(|l| l.display_name.as_str()).collect();
                        println!("  Labels:   {}", labels.join(", "));
                    }
                    println!("  Created:  {}", skill.created_at);
                    println!("  Updated:  {}", skill.updated_at);
                }
                None => eprintln!("{}", "Skill not found.".yellow()),
            }
        }

        Commands::Versions { namespace, slug, page, page_size } => {
            let (service, _) = create_service(registry).await?;
            let versions = service.list_versions(&namespace, &slug, page, page_size).await?;
            println!("{}", format!("Versions for {}/{} (page {}):", namespace, slug, page).cyan());
            for v in &versions {
                let status_color = match v.status.as_str() {
                    "active" => v.status.green(),
                    "yanked" => v.status.red(),
                    _ => v.status.normal(),
                };
                println!(
                    "  v{} [{}] files:{} size:{} published:{} by:{}",
                    v.version,
                    status_color,
                    v.file_count,
                    v.total_size,
                    v.published_at.as_deref().unwrap_or("-"),
                    v.create_by,
                );
            }
            if versions.is_empty() {
                println!("  No versions found.");
            }
        }

        Commands::Download { namespace, slug, version, output } => {
            let (service, _) = create_service(registry).await?;
            let data = if let Some(ver) = &version {
                service.download_bundle(&namespace, &slug, ver).await?
            } else {
                service.download_latest(&namespace, &slug).await?
            };
            match output {
                Some(path) => {
                    std::fs::write(&path, &data)?;
                    eprintln!("{}", format!("Downloaded {} bytes to {}", data.len(), path).green());
                }
                None => {
                    use std::io::Write;
                    std::io::stdout().write_all(&data)?;
                }
            }
        }

        Commands::Publish { namespace, file, visibility } => {
            let (_, config) = create_service(registry).await?;
            let file_data = std::fs::read(&file)?;
            let _vis: domain::value_object::visibility::Visibility = visibility.as_str().try_into()
                .map_err(|e: String| -> Box<dyn std::error::Error> { e.into() })?;
            let client = Arc::new(SkillHubClient::new(config.registry.url.clone(), config.auth.token.clone()));
            let filename = std::path::Path::new(&file).file_name().unwrap().to_str().unwrap();
            match client.publish(&namespace, file_data, filename, visibility).await {
                Ok(result) => {
                    println!("{}", "Published successfully!".green());
                    println!("  Skill: {}/{}", result.namespace, result.slug);
                    println!("  Version: {}", result.version);
                    println!("  Status: {}", result.status);
                    println!("  Files: {} ({} bytes)", result.file_count, result.total_size);
                }
                Err(e) => eprintln!("{} {}", "Publish failed:".red(), e),
            }
            return Ok(());
        }

        Commands::Star { skill_id } => {
            let (service, _) = create_service(registry).await?;
            service.star_skill(&skill_id).await?;
            println!("{}", "Starred!".green());
        }

        Commands::Unstar { skill_id } => {
            let (service, _) = create_service(registry).await?;
            service.unstar_skill(&skill_id).await?;
            println!("{}", "Unstarred.".green());
        }

        Commands::Rate { skill_id, score } => {
            if score < 1 || score > 5 {
                eprintln!("{}", "Score must be between 1 and 5.".red());
                return Ok(());
            }
            let (service, _) = create_service(registry).await?;
            service.rate_skill(&skill_id, score).await?;
            println!("{}", format!("Rated {} \u{2605}{}", score, skill_id).green());
        }

        Commands::Stats => {
            let (service, _) = create_service(registry).await?;
            let stats = service.get_stats().await?;
            println!("{}", "SkillHub Statistics".cyan().bold());
            println!("  Skills:     {}", stats.total_skills);
            println!("  Downloads:  {}", stats.total_downloads);
            println!("  Namespaces: {}", stats.total_namespaces);
            println!("  Ratings:    {}", stats.total_ratings);
        }

        Commands::Namespaces => {
            let (service, _) = create_service(registry).await?;
            let namespaces = service.list_namespaces().await?;
            println!("{}", "Namespaces:".cyan());
            for ns in &namespaces {
                println!(
                    "  {} [{}] {} (members: {}, skills: {})",
                    ns.slug,
                    ns.namespace_type,
                    ns.display_name,
                    ns.member_count,
                    ns.skill_count,
                );
                if let Some(desc) = &ns.description {
                    println!("    {}", desc.dimmed());
                }
            }
            if namespaces.is_empty() {
                println!("  No namespaces found.");
            }
        }

        Commands::Labels => {
            let (service, _) = create_service(registry).await?;
            let labels = service.list_labels().await?;
            println!("{}", "Labels:".cyan());
            for label in &labels {
                println!("  {} ({}) - {}", label.slug, label.label_type, label.display_name);
            }
            if labels.is_empty() {
                println!("  No labels found.");
            }
        }

        Commands::MySkills { page, page_size } => {
            let (service, _) = create_service(registry).await?;
            let (skills, total) = service.list_my_skills(page, page_size).await?;
            println!("{}", format!("My Skills ({} total, page {}):", total, page).cyan());
            for skill in &skills {
                let version = skill.latest_version.as_deref().unwrap_or("-");
                println!("  {}/{} v{} [{}]", skill.namespace_slug, skill.slug, version, skill.status);
            }
            if skills.is_empty() {
                println!("  No skills found.");
            }
        }

        Commands::MyStars { page, page_size } => {
            let (service, _) = create_service(registry).await?;
            let (skills, total) = service.list_my_stars(page, page_size).await?;
            println!("{}", format!("My Stars ({} total, page {}):", total, page).cyan());
            for skill in &skills {
                println!("  {}/{} \u{2605}{} downloads:{}", skill.namespace_slug, skill.slug, skill.star_count, skill.download_count);
            }
            if skills.is_empty() {
                println!("  No starred skills.");
            }
        }

        Commands::Profile => {
            let (service, _) = create_service(registry).await?;
            match service.get_user_profile().await? {
                Some(profile) => {
                    println!("{}", "Profile:".cyan().bold());
                    println!("  ID:     {}", profile.id);
                    if let Some(email) = &profile.email {
                        println!("  Email:  {}", email);
                    }
                    println!("  Name:   {}", profile.display_name);
                    if let Some(avatar) = &profile.avatar_url {
                        println!("  Avatar: {}", avatar);
                    }
                    println!("  Status: {}", profile.status);
                }
                None => eprintln!("{}", "Not authenticated. Run `skillhub login` first.".yellow()),
            }
        }

        Commands::Notifications { page, page_size } => {
            let (service, _) = create_service(registry).await?;
            let notifications = service.list_notifications(None, page, page_size).await?;
            println!("{}", format!("Notifications (page {}):", page).cyan());
            for n in &notifications {
                let marker = if n.is_read() { " " } else { "\u{25cf}" };
                println!("  {} [{}] {} - {}", marker, n.notification_type, n.title, n.created_at);
            }
            if notifications.is_empty() {
                println!("  No notifications.");
            }
        }

        Commands::ReadNotification { id } => {
            let (service, _) = create_service(registry).await?;
            service.mark_notification_read(&id).await?;
            println!("{}", "Notification marked as read.".green());
        }

        Commands::ReadAllNotifications => {
            let (service, _) = create_service(registry).await?;
            service.mark_all_notifications_read().await?;
            println!("{}", "All notifications marked as read.".green());
        }

        Commands::Tokens { page, page_size } => {
            let (service, _) = create_service(registry).await?;
            let tokens = service.list_tokens(page, page_size).await?;
            println!("{}", format!("API Tokens (page {}):", page).cyan());
            for t in &tokens {
                let expires = t.expires_at.as_deref().unwrap_or("never");
                println!("  {} [{}] scopes:{} created:{} expires:{}", t.name, t.id, t.scopes.join(","), t.created_at, expires);
            }
            if tokens.is_empty() {
                println!("  No tokens found.");
            }
        }

        Commands::DeleteToken { id } => {
            let (service, _) = create_service(registry).await?;
            service.delete_token(&id).await?;
            println!("{}", "Token deleted.".green());
        }

        Commands::Whoami => {
            let (service, _) = create_service(registry).await?;
            match service.get_user_profile().await? {
                Some(profile) => {
                    println!("{}", "Logged in as:".green().bold());
                    println!("  Username: {}", profile.display_name);
                    if let Some(email) = &profile.email {
                        println!("  Email:    {}", email.dimmed());
                    }
                    println!("  Status:   {}", profile.status);
                    println!("  User ID:  {}", profile.id);
                }
                None => eprintln!("{}", "Not authenticated. Run `skillhub login` first.".yellow()),
            }
        }

        Commands::Explore { limit, sort } => {
            let (service, _) = create_service(registry).await?;
            let (skills, total) = service.search_skills(None, None, Vec::new(), sort, 1, limit).await?;
            println!("{}", format!("Latest skills ({} total):", total).cyan().bold());
            for skill in &skills {
                let version = skill.latest_version.as_deref().unwrap_or("-");
                let updated = skill.updated_at.split('T').next().unwrap_or(&skill.updated_at);
                println!(
                    "  {}/{} v{} {} {}",
                    skill.namespace_slug.bold(),
                    skill.slug,
                    version.dimmed(),
                    skill.status,
                    updated.dimmed()
                );
                if let Some(summary) = &skill.summary {
                    let truncated = if summary.len() > 60 {
                        format!("{}...", &summary[..60])
                    } else {
                        summary.clone()
                    };
                    println!("    {}", truncated.dimmed());
                }
            }
            if skills.is_empty() {
                println!("  No skills found.");
            }
        }

        Commands::Inspect { namespace, slug, versions, files, file, version } => {
            let (service, _) = create_service(registry).await?;

            // Get skill detail
            match service.get_skill_detail(&namespace, &slug).await? {
                Some(skill) => {
                    println!("{}", format!("{}: {}", skill.namespace_slug, skill.slug).cyan().bold());
                    if let Some(name) = &skill.display_name {
                        println!("  Name:     {}", name);
                    }
                    if let Some(summary) = &skill.summary {
                        println!("  Summary:  {}", summary);
                    }
                    println!("  Owner:    {}", skill.owner_name);
                    println!("  Status:   {}", skill.status);
                    let version = version.as_deref().or(skill.latest_version.as_deref()).unwrap_or("-");
                    println!("  Version:  {}", version);
                    println!("  Downloads: {}", skill.download_count);
                    println!("  Stars:    {}", skill.star_count);
                    if skill.rating_count > 0 {
                        println!("  Rating:   {:.1}/5 ({})", skill.rating_avg, skill.rating_count);
                    }
                    if !skill.tags.is_empty() {
                        println!("  Tags:     {}", skill.tags.join(", "));
                    }
                    println!("  Updated:  {}", skill.updated_at);
                }
                None => eprintln!("{}", "Skill not found.".red()),
            }

            // List versions if requested
            if versions {
                let vers = service.list_versions(&namespace, &slug, 1, 100).await?;
                println!("\n{}", "Versions:".cyan().bold());
                for v in &vers {
                    let status_color = match v.status.as_str() {
                        "active" => v.status.green(),
                        "yanked" => v.status.red(),
                        _ => v.status.normal(),
                    };
                    println!(
                        "  v{} [{}] files:{} size:{}",
                        v.version,
                        status_color,
                        v.file_count,
                        v.total_size
                    );
                }
            }

            // List files if requested
            if files {
                let ver = version.as_deref();
                let skill_ver = service.get_skill_detail(&namespace, &slug).await?.ok_or("Skill not found")?;
                let ver_str = ver.or(skill_ver.latest_version.as_deref()).ok_or("No version available")?;
                let file_list = service.list_files(&namespace, &slug, ver_str).await?;
                println!("\n{}", format!("Files (v{}):", ver_str).cyan().bold());
                for f in &file_list {
                    println!("  {} ({} bytes)", f.file_path, f.file_size);
                }
            }

            // Show file content if requested
            if let Some(file_path) = file {
                let ver = version.as_deref();
                let skill_ver = service.get_skill_detail(&namespace, &slug).await?.ok_or("Skill not found")?;
                let ver_str = ver.or(skill_ver.latest_version.as_deref()).ok_or("No version available")?;
                let content = service.get_file_content(&namespace, &slug, ver_str, &file_path).await?;
                println!("\n{}", format!("{}:{}:", ver_str, file_path).cyan().bold());
                for line in content.lines().take(100) {
                    println!("  {}", line);
                }
            }
        }

        Commands::Report { namespace, slug, reason, description } => {
            let (service, _) = create_service(registry).await?;
            service.submit_report(&namespace, &slug, &reason, description.as_deref()).await?;
            println!("{}", "Report submitted. Thank you for helping keep SkillHub safe!".green());
        }

        Commands::Install { namespace, slug, version, yes: _yes } => {
            use infrastructure::local_store::LocalStore;

            let (service, _) = create_service(registry).await?;
            let mut local_store = LocalStore::new()?;

            // 获取技能详情以确定版本
            let skill_detail = service.get_skill_detail(&namespace, &slug).await?
                .ok_or_else(|| format!("Skill {}/{} not found", namespace, slug))?;

            let install_version = version.as_deref()
                .or(skill_detail.latest_version.as_deref())
                .ok_or("No version available")?;

            println!("{}", format!("Installing {}/{} v{}...", namespace, slug, install_version).cyan().bold());

            // 下载技能包
            let data = service.download_bundle(&namespace, &slug, install_version).await?;

            // 安装到本地
            let skill_dir = local_store.install(&namespace, &slug, install_version, data).await?;

            println!("{} {}", "✅".green(), format!("Installed to: {}", skill_dir.display()).green());

            // 如果是 Rust 项目，检查环境
            if let Some(installed) = local_store.get_installed(&namespace, &slug) {
                if matches!(installed.skill_type, infrastructure::local_store::SkillType::Rust) {
                    println!();
                    println!("{}", "🦀 Rust project detected!".cyan().bold());

                    // 检测是否在国内（通过检查系统语言或使用 iana_time_zone）
                    let is_china = std::env::var("LANG")
                        .ok()
                        .filter(|l| l.contains("zh_CN") || l.contains("zh_CN.UTF-8"))
                        .is_some();

                    if is_china {
                        let _ = local_store.setup_china_mirror();
                    }

                    // 询问是否构建
                    use dialoguer::Confirm;
                    let should_build = Confirm::new()
                        .with_prompt("Build this Rust project now? (cargo build --release)")
                        .default(false)
                        .interact()?;

                    if should_build {
                        match local_store.build_rust_project(&skill_dir) {
                            Ok(_) => println!("{} {}", "✅".green(), "Build complete!".green()),
                            Err(e) => eprintln!("{} {}", "⚠".yellow(), format!("Build skipped: {}", e)),
                        }
                    } else {
                        println!("{} You can build it later with:", "ℹ".cyan());
                        println!("   cd {}", skill_dir.display());
                        println!("   cargo build --release");
                    }
                }
            }
        }

        Commands::Uninstall { namespace, slug, yes } => {
            use infrastructure::local_store::LocalStore;
            use dialoguer::Confirm;

            let mut local_store = LocalStore::new()?;

            // 检查是否已安装
            let installed = local_store.get_installed(&namespace, &slug);
            if installed.is_none() {
                eprintln!("{}", format!("Skill {}/{} is not installed", namespace, slug).yellow());
                return Ok(());
            }

            let installed = installed.unwrap();

            // 确认卸载
            if !yes {
                let should_remove = Confirm::new()
                    .with_prompt(&format!("Uninstall {}/{} (v{})?", namespace, slug, installed.version))
                    .default(false)
                    .interact()?;

                if !should_remove {
                    println!("{}", "Uninstall cancelled".dimmed());
                    return Ok(());
                }
            }

            local_store.uninstall(&namespace, &slug).await?;
            println!("{}", format!("✅ Uninstalled {}/{}", namespace, slug).green());
        }

        Commands::List => {
            use infrastructure::local_store::LocalStore;

            let local_store = LocalStore::new()?;
            let installed = local_store.list()?;

            if installed.is_empty() {
                println!("{}", "No skills installed yet.".dimmed());
                println!("{}", "Install a skill with: skillhub install <namespace> <slug>".dimmed());
            } else {
                println!("{}", format!("Installed skills ({}):", installed.len()).cyan().bold());
                for skill in &installed {
                    let type_icon = match skill.skill_type {
                        infrastructure::local_store::SkillType::Rust => "🦀",
                        infrastructure::local_store::SkillType::JavaScript => "📦",
                        infrastructure::local_store::SkillType::Python => "🐍",
                        infrastructure::local_store::SkillType::Archive => "📦",
                    };
                    let line = format!("  {} {}/{} v{}",
                        type_icon, skill.namespace, skill.slug, skill.version);
                    println!("{}", line);
                    println!("    {}", skill.install_path.display().to_string().dimmed());
                }
            }
        }

        Commands::Update { skill, all } => {
            use infrastructure::local_store::LocalStore;

            let (service, _) = create_service(registry).await?;
            let mut local_store = LocalStore::new()?;
            let installed = local_store.list()?;

            if installed.is_empty() {
                eprintln!("{}", "No skills installed yet.".yellow());
                return Ok(());
            }

            // 确定要更新的技能
            let skills_to_update: Vec<_> = if all {
                installed.iter().collect()
            } else if let Some(skill_ref) = skill {
                let parts: Vec<&str> = skill_ref.split('/').collect();
                if parts.len() != 2 {
                    eprintln!("{}", format!("Invalid skill format: '{}'. Use 'namespace/slug'", skill_ref).red());
                    return Ok(());
                }
                let (ns, sl) = (parts[0], parts[1]);
                installed.iter().filter(|s| s.namespace == ns && s.slug == sl).collect()
            } else {
                eprintln!("{}", "Please specify --all or a specific skill (e.g., my-namespace/my-skill)".yellow());
                return Ok(());
            };

            if skills_to_update.is_empty() {
                println!("{}", "No matching skills found to update.".dimmed());
                return Ok(());
            }

            println!("{}", format!("Checking updates for {} skill(s)...", skills_to_update.len()).cyan().bold());

            for installed_skill in skills_to_update {
                println!();
                println!("{}", format!("Checking {}/{}...", installed_skill.namespace, installed_skill.slug).cyan());

                // 获取最新版本
                match service.get_skill_detail(&installed_skill.namespace, &installed_skill.slug).await? {
                    Some(detail) => {
                        let latest_version = detail.latest_version.as_deref().unwrap_or("unknown");

                        if latest_version == installed_skill.version {
                            println!("  {} Already up-to-date (v{})", "✅".green(), latest_version);
                        } else {
                            println!("  {} Update available: v{} -> v{}", "⬆".yellow(), installed_skill.version, latest_version);

                            // 下载并更新
                            match service.download_bundle(&installed_skill.namespace, &installed_skill.slug, latest_version).await {
                                Ok(data) => {
                                    match local_store.update(&installed_skill.namespace, &installed_skill.slug, latest_version, data).await {
                                        Ok(_) => println!("  {} Updated to v{}", "✅".green(), latest_version),
                                        Err(e) => eprintln!("  {} Failed to update: {}", "❌".red(), e),
                                    }
                                }
                                Err(e) => eprintln!("  {} Failed to download: {}", "❌".red(), e),
                            }
                        }
                    }
                    None => {
                        println!("  {} Skill not found on hub", "⚠".yellow());
                    }
                }
            }

            println!();
            println!("{}", "✅ Update check complete!".green());
        }

        Commands::Skill(skill_cmd) => {
            run_skill_command(skill_cmd, registry).await?;
        }

        Commands::Review(review_cmd) => {
            run_review_command(review_cmd, registry).await?;
        }

        Commands::Transfer(transfer_cmd) => {
            run_transfer_command(transfer_cmd, registry).await?;
        }

        Commands::Admin(admin_cmd) => {
            run_admin_command(admin_cmd, registry).await?;
        }

        _ => unreachable!(),
    }

    Ok(())
}

async fn run_skill_command(cmd: SkillCommands, registry: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let (service, _) = create_service(registry).await?;

    match cmd {
        SkillCommands::Delete { namespace, slug } | SkillCommands::Hide { namespace, slug } => {
            match service.archive(&namespace, &slug).await {
                Ok(resp) => println!("{}", format!("Skill {} deleted: {}", slug, resp.new_status).green()),
                Err(e) => eprintln!("{} {}", "Failed to delete skill:".red(), e),
            }
        }

        SkillCommands::Undelete { namespace, slug } | SkillCommands::Unhide { namespace, slug } => {
            match service.unarchive(&namespace, &slug).await {
                Ok(resp) => println!("{}", format!("Skill {} restored: {}", slug, resp.new_status).green()),
                Err(e) => eprintln!("{} {}", "Failed to restore skill:".red(), e),
            }
        }

        SkillCommands::Rename { namespace, slug, new_slug } => {
            // TODO: Implement rename API
            println!("{} Renaming {} to {}...", "Pending backend API:".yellow(), slug, new_slug);
        }

        SkillCommands::Yank { namespace, slug, version, reason } => {
            let r = reason.as_deref().unwrap_or("No reason provided");
            match service.yank_version(&namespace, &slug, &version, r).await {
                Ok(resp) => println!("{}", format!("Version {} yanked: {}", version, resp.new_status).green()),
                Err(e) => eprintln!("{} {}", "Failed to yank version:".red(), e),
            }
        }

        SkillCommands::Rerelease { namespace, slug, version } => {
            match service.rerelease_version(&namespace, &slug, &version).await {
                Ok(resp) => println!("{}", format!("Version {} re-released: {}", version, resp.new_status).green()),
                Err(e) => eprintln!("{} {}", "Failed to rerelease version:".red(), e),
            }
        }
    }

    Ok(())
}

async fn run_review_command(cmd: ReviewCommands, registry: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let (service, _) = create_service(registry).await?;

    match cmd {
        ReviewCommands::Submit { namespace, slug, version, comment } => {
            match service.submit_review(&namespace, &slug, &version, comment.as_deref()).await {
                Ok(resp) => println!("{}", format!("Review submitted: {}", resp.new_status).green()),
                Err(e) => eprintln!("{} {}", "Failed to submit review:".red(), e),
            }
        }

        ReviewCommands::Withdraw { namespace, slug, version } => {
            match service.withdraw_review(&namespace, &slug, &version).await {
                Ok(resp) => println!("{}", format!("Review withdrawn: {}", resp.new_status).green()),
                Err(e) => eprintln!("{} {}", "Failed to withdraw review:".red(), e),
            }
        }

        ReviewCommands::List { status } => {
            let reviews = service.list_reviews(status.as_deref()).await?;
            let status_text = status.as_ref().map(|s| format!(" ({})", s)).unwrap_or_default();
            println!("{}", format!("Review tasks{}:", status_text).cyan());
            for r in &reviews {
                println!("  {} [{}] {}", r.id, r.status, r.created_at);
                if let Some(c) = &r.comment {
                    println!("    {}", c.dimmed());
                }
            }
            if reviews.is_empty() {
                println!("  No review tasks found.");
            }
        }
    }

    Ok(())
}

async fn run_transfer_command(cmd: TransferCommands, _registry: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        TransferCommands::Request { namespace, slug, target_handle, message } => {
            println!("{} Transfer request for {}/{} to {} ({:?})",
                "Pending backend API:".yellow(),
                namespace, slug, target_handle, message
            );
        }

        TransferCommands::List { outgoing } => {
            println!("{} Listing {} transfer requests...",
                "Pending backend API:".yellow(),
                if outgoing { "outgoing" } else { "incoming" }
            );
        }

        TransferCommands::Accept { id } => {
            println!("{} Accepting transfer request {}...", "Pending backend API:".yellow(), id);
        }

        TransferCommands::Reject { id } => {
            println!("{} Rejecting transfer request {}...", "Pending backend API:".yellow(), id);
        }

        TransferCommands::Cancel { id } => {
            println!("{} Cancelling transfer request {}...", "Pending backend API:".yellow(), id);
        }
    }

    Ok(())
}

async fn run_admin_command(cmd: AdminCommands, _registry: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        AdminCommands::BanUser { handle, reason, id } => {
            println!("{} Banning {} ({:?})...",
                "Pending backend API:".yellow(),
                if id { "ID" } else { "handle" },
                handle
            );
            if let Some(r) = reason {
                println!("  Reason: {}", r);
            }
        }

        AdminCommands::SetRole { handle, role, id } => {
            println!("{} Setting role for {} to {} ({:?})...",
                "Pending backend API:".yellow(),
                if id { "ID" } else { "handle" },
                handle, role
            );
        }
    }

    Ok(())
}

async fn run_init() -> Result<(), Box<dyn std::error::Error>> {
    use dialoguer::{Input, Select};

    println!("{}", "\u{2554}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2557}".cyan());
    println!("{}", "\u{2551}   SkillHub Configuration Wizard      \u{2551}".cyan());
    println!("{}", "\u{255a}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{255d}".cyan());
    println!();

    let default_url = "http://localhost:3002";
    let registry_url: String = Input::new()
        .with_prompt("Registry URL")
        .default(default_url.to_string())
        .interact_text()?;

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

            let client = SkillHubClient::new(config.registry.url.clone(), None);
            match client.login(&username, &password).await {
                Ok(tokens) => {
                    config.auth.token = Some(tokens.access_token);
                    config.auth.refresh_token = tokens.refresh_token;
                    println!("\n{}", "\u{2713} Login successful!".green());
                }
                Err(e) => {
                    println!("\n{} {}", "\u{2717} Login failed:".red(), e);
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

            let client = SkillHubClient::new(config.registry.url.clone(), None);
            match client.login_with_app(&app_id, &app_secret).await {
                Ok(tokens) => {
                    config.auth.token = Some(tokens.access_token);
                    config.auth.refresh_token = tokens.refresh_token;
                    config.auth.app_id = Some(app_id);
                    config.auth.app_secret = Some(app_secret);
                    println!("\n{}", "\u{2713} App authentication successful!".green());
                }
                Err(e) => {
                    println!("\n{} {}", "\u{2717} App authentication failed:".red(), e);
                    println!("{}", "  Config saved without token. You can login later in TUI.".yellow());
                }
            }
        }
        _ => {
            println!("\n{}", "\u{2139} Skipping authentication.".dimmed());
        }
    }

    config::save(&config)?;

    let config_path = dirs::home_dir()
        .map(|h| h.join(".skillhub").join("config.toml"))
        .map(|p| p.display().to_string())
        .unwrap_or_default();

    println!("\n{} {}", "\u{2713} Config saved to".green(), config_path.bold());
    println!("{}", "  Run `skillhub` to start the TUI.".dimmed());

    Ok(())
}

async fn run_tui(registry_override: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = config::load()?;

    if let Some(url) = registry_override {
        config.registry.url = url;
    }

    let has_app_credentials = config.auth.has_app_credentials();
    let registry_url = config.registry.url.clone();

    let client = Arc::new(SkillHubClient::new(
        registry_url.clone(),
        config.auth.token.clone(),
    ));

    if has_app_credentials && config.auth.token.is_none() {
        if let (Some(app_id), Some(app_secret)) = (&config.auth.app_id, &config.auth.app_secret) {
            match client.login_with_app(app_id, app_secret).await {
                Ok(tokens) => {
                    client.set_tokens(tokens.access_token.clone(), tokens.refresh_token.clone());
                    config.auth.token = Some(tokens.access_token);
                    config.auth.refresh_token = tokens.refresh_token;
                }
                Err(_) => {}
            }
        }
    }

    let auth_repo = Arc::new(AuthRepositoryImpl::new(client.clone()))
        as Arc<dyn domain::repository::auth_repository::AuthRepository>;
    let skill_repo = Arc::new(SkillRepositoryImpl::new(client.clone()))
        as Arc<dyn domain::repository::skill_repository::SkillRepository>;

    let service = Arc::new(
        SkillHubService::new(skill_repo).with_auth_repo(auth_repo),
    );

    let is_authenticated = client.has_token();

    let _should_save_config = tui::runner::run(
        service,
        client,
        is_authenticated,
        registry_url,
    ).await?;

    Ok(())
}
