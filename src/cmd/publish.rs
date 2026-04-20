use crate::client::ApiClient;
use crate::config::load;
use crate::error::Result;
use clap::Parser;
use colored::Colorize;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct PublishArgs {
    pub path: PathBuf,

    #[arg(long)]
    pub namespace: Option<String>,

    #[arg(long, default_value = "private")]
    pub visibility: String,
}

pub async fn run(args: PublishArgs) -> Result<()> {
    let config = load()?;
    let token = config.auth.token.ok_or(crate::error::CliError::NotAuthenticated)?;

    if !args.path.exists() {
        return Err(crate::error::CliError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("File not found: {}", args.path.display()),
        )));
    }

    let namespace = if let Some(ns) = args.namespace {
        ns
    } else {
        dialoguer::Input::<String>::new()
            .with_prompt("Namespace")
            .interact()?
    };

    println!(
        "{}",
        format!(
            "Publishing {} to namespace {}...",
            args.path.display().to_string().bright_white(),
            namespace.cyan()
        )
        .bright_cyan()
    );

    let client = ApiClient::new(config.registry.url, Some(token));
    let result = client.publish(&namespace, &args.path).await?;

    println!("{}", "Published successfully!".green());
    println!("{:#}", serde_json::to_string_pretty(&result)?);

    Ok(())
}
