mod application;
mod domain;
mod infrastructure;
mod tui;

use application::service::SkillHubService;
use infrastructure::config::{load, save};
use infrastructure::client::SkillHubClient;
use infrastructure::repository::{auth_repository_impl::AuthRepositoryImpl, skill_repository_impl::SkillRepositoryImpl};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = load()?;

    // Check authentication
    let token = config.auth.token.clone();
    let registry_url = config.registry.url.clone();

    // Create HTTP client (may be without token initially)
    let client = Arc::new(SkillHubClient::new(
        registry_url.clone(),
        token.clone(),
    ));

    // Create auth repository (always available)
    let auth_repo = Arc::new(AuthRepositoryImpl::new(client.clone())) as Arc<dyn domain::repository::auth_repository::AuthRepository>;

    // Create skill repository
    let skill_repo = Arc::new(SkillRepositoryImpl::new(client.clone())) as Arc<dyn domain::repository::skill_repository::SkillRepository>;

    // TODO: Add other repository implementations

    // Create application service
    let service = Arc::new(
        SkillHubService::new(skill_repo)
            .with_auth_repo(auth_repo)
    );

    // Run TUI
    let should_save_config = tui::runner::run(
        service,
        token.is_none(), // is_first_login
        registry_url,
    ).await?;

    // Save config if login was successful
    if should_save_config {
        // Reload config to get the token
        let updated_config = load()?;
        save(&updated_config)?;
    }

    Ok(())
}
