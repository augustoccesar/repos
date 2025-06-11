mod commands;
mod repository;

use std::process::exit;

use clap::{Parser, Subcommand};

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

    #[clap(about = "Move repository under the repos tracked structure")]
    Track(commands::TrackArgs),

    #[clap(about = "Updates the CLI to the latest available version")]
    Update(commands::UpdateArgs),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let status = match &cli.command {
        Commands::Activate(args) => commands::handle_activate(args).await,
        Commands::Expand(args) => commands::handle_expand(args).await,
        Commands::List(args) => commands::handle_list(args).await,
        Commands::Track(args) => commands::handle_track(args).await,
        Commands::Update(args) => commands::handle_update(args).await,
    }?;

    exit(status);
}
