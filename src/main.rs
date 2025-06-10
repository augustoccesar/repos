use std::process::exit;

use clap::{Parser, Subcommand};

mod commands;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(about = "Expands the passed on repository name to the full path")]
    Expand(commands::ExpandArgs),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let status = match &cli.command {
        Commands::Expand(args) => commands::handle_expand(args).await,
    }?;

    exit(status);
}
