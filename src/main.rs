mod commands;
mod config;
mod error;
mod repo_name;

use std::process::exit;

use clap::{command, Parser, Subcommand};

use commands::{CleanupCommandArgs, ConfigCommandArgs, ExpandCommandArgs, SetupCommandArgs};
use config::Config;
use error::{Error, Result};

const EXIT_STATUS_ABORTED: i32 = 1;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[command()]
    Expand(ExpandCommandArgs),
    #[command()]
    Setup(SetupCommandArgs),
    #[command()]
    Config(ConfigCommandArgs),
    #[command()]
    Cleanup(CleanupCommandArgs),
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut config = Config::load()?;

    let result = match cli.command {
        Command::Expand(args) => commands::expand(args, &config),
        Command::Setup(args) => commands::setup(args),
        Command::Config(args) => commands::config(args, &mut config),
        Command::Cleanup(args) => commands::cleanup(args),
    };

    match result {
        Ok(_) => return Ok(()),
        Err(err) => match err {
            Error::Aborted => {
                println!("Aborted!");
                exit(EXIT_STATUS_ABORTED);
            }
            err => return Err(err),
        },
    }
}
