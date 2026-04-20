use crate::client::ApiClient;
use crate::config::{load, save};
use crate::error::Result;
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Input, Password};

pub async fn run(url: Option<String>) -> Result<()> {
    let mut config = load()?;
    let registry_url = url.unwrap_or_else(|| config.registry.url.clone());

    println!("{}", "SkillHub Login".bright_cyan().bold());
    println!("{}", "=".repeat(20));

    let username = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Username")
        .interact()?;

    let password = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Password")
        .interact()?;

    let mut client = ApiClient::new(registry_url.clone(), None);
    let resp = client.login(&username, &password).await?;

    config.registry.url = registry_url;
    config.auth.token = Some(resp.token);
    save(&config)?;

    println!("{}", "Login successful!".green());
    Ok(())
}
