mod commands;
mod config;
mod git;
mod repos_dir;

use anyhow::Context;
use clap::{Parser, Subcommand};

use crate::config::Config;
use crate::repos_dir::list_repositories;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(about = "Export helpers for the shell")]
    Activate(commands::ActivateArgs),

    #[clap(about = "Expands the passed on repository name to the full path")]
    Expand(commands::ExpandArgs),

    #[clap(about = "List all the available repositories")]
    List(commands::ListArgs),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let config = Config::load().context("failed to load config file")?;

    match &cli.command {
        Commands::Activate(args) => commands::handle_activate(args).await,
        Commands::Expand(args) => commands::handle_expand(args, &config).await,
        Commands::List(args) => commands::handle_list(args, &config).await,
    }

    Ok(())
}
