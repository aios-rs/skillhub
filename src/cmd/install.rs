use crate::client::ApiClient;
use crate::config::load;
use crate::error::Result;
use clap::Parser;
use colored::Colorize;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct InstallArgs {
    pub skill_spec: String,

    #[arg(short, long)]
    pub output: Option<PathBuf>,
}

fn parse_skill_spec(spec: &str) -> Result<(String, String, Option<String>)> {
    let parts: Vec<&str> = spec.split('/').collect();
    if parts.len() < 2 {
        return Err(crate::error::CliError::Parse(format!(
            "Invalid skill spec. Expected: namespace/slug[@version], got: {}",
            spec
        )));
    }

    let namespace = parts[0].to_string();
    let slug_and_ver = parts[1];
    let (slug, version) = if let Some(idx) = slug_and_ver.find('@') {
        (
            slug_and_ver[..idx].to_string(),
            Some(slug_and_ver[idx + 1..].to_string()),
        )
    } else {
        (slug_and_ver.to_string(), None)
    };

    Ok((namespace, slug, version))
}

pub async fn run(args: InstallArgs) -> Result<()> {
    let config = load()?;
    let token = config.auth.token.ok_or(crate::error::CliError::NotAuthenticated)?;

    let (namespace, slug, version_opt) = parse_skill_spec(&args.skill_spec)?;

    let client = ApiClient::new(config.registry.url, Some(token));

    let version = if let Some(v) = version_opt {
        v
    } else {
        let detail = client.get_skill_detail(&namespace, &slug).await?;
        detail["latest_version"]
            .as_str()
            .ok_or_else(|| crate::error::CliError::Parse("No latest version found".to_string()))?
            .to_string()
    };

    println!(
        "{}",
        format!(
            "Downloading {} {} {}...",
            namespace.cyan(),
            slug.bright_white(),
            format!("(v{})", version.dimmed())
        )
        .bright_cyan()
    );

    let bytes = client
        .download_bundle(&namespace, &slug, &version)
        .await?;

    let output_dir = args.output.unwrap_or_else(|| PathBuf::from("."));
    let filename = format!("{}-{}-{}.zip", namespace, slug, version);
    let output_path = output_dir.join(&filename);

    std::fs::write(&output_path, bytes)?;

    println!(
        "{} {}",
        "Downloaded to:".green(),
        output_path.display().to_string().bright_white()
    );

    Ok(())
}
