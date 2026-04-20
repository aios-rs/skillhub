mod client;
mod cmd;
mod config;
mod error;

use clap::{Parser, Subcommand};
use colored::Colorize;

#[derive(Parser)]
#[command(name = "skillhub")]
#[command(about = "CLI for SkillHub - AI Agent Skill Registry", long_about = None)]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Login {
        #[arg(short, long)]
        url: Option<String>,
    },
    Search {
        query: String,
        #[arg(short, long, default_value_t = 20)]
        limit: i32,
        #[arg(long)]
        sort: Option<String>,
    },
    Publish {
        path: String,
        #[arg(long)]
        namespace: Option<String>,
        #[arg(long, default_value = "private")]
        visibility: String,
    },
    Install {
        skill_spec: String,
        #[arg(short, long)]
        output: Option<String>,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Login { url } => cmd::login::run(url).await,
        Commands::Search { query, limit, sort } => {
            cmd::search::run(cmd::search::SearchArgs { query, limit, sort }).await
        }
        Commands::Publish { path, namespace, visibility } => {
            cmd::publish::run(cmd::publish::PublishArgs {
                path: path.into(),
                namespace,
                visibility,
            })
            .await
        }
        Commands::Install { skill_spec, output } => {
            cmd::install::run(cmd::install::InstallArgs {
                skill_spec,
                output: output.map(|p| p.into()),
            })
            .await
        }
    };

    if let Err(e) = result {
        eprintln!("{} {}", "error:".red().bold(), e);
        std::process::exit(1);
    }
}
