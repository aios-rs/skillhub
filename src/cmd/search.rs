use crate::client::ApiClient;
use crate::config::load;
use crate::error::Result;
use clap::Parser;
use colored::Colorize;

#[derive(Parser, Debug)]
pub struct SearchArgs {
    pub query: String,

    #[arg(short, long, default_value_t = 20)]
    pub limit: i32,

    #[arg(long)]
    pub sort: Option<String>,
}

pub async fn run(args: SearchArgs) -> Result<()> {
    let config = load()?;
    let token = config.auth.token.ok_or(crate::error::CliError::NotAuthenticated)?;

    let client = ApiClient::new(config.registry.url, Some(token));
    let resp = client.search(&args.query, 1, args.limit, args.sort.as_deref()).await?;

    println!(
        "{}\n",
        format!("Found {} skills for \"{}\"", resp.total, args.query.bright_white())
            .bright_cyan()
            .bold()
    );

    for skill in resp.items {
        println!("{} / {}", skill.namespace.cyan(), skill.name.bright_white().bold());
        println!(
            "  {} {}",
            "v:".dimmed(),
            format!(
                "{} | {} | {}",
                skill.version,
                format!("{:.1}", skill.avg_rating).yellow(),
                format!("{} downloads", skill.downloads).dimmed()
            )
        );
        println!("  {}", skill.description.dimmed());
        println!();
    }

    Ok(())
}
