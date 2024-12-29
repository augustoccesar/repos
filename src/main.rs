mod commands;
mod config;
mod error;
mod repo_name;
mod shell;

use std::process::exit;

use clap::{command, Parser, Subcommand};

use commands::{
    CleanupCommandArgs, ConfigCommandArgs, ExpandCommandArgs, ListCommandArgs, NewCommandArgs, SetupCommandArgs, TrackCommandArgs
};
use config::Config;
use error::{Error, Result};

const EXIT_STATUS_ABORTED: i32 = 1;
const EXIT_STATUS_NEED_CLONE: i32 = 8;

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
    #[command()]
    New(NewCommandArgs),
    #[command()]
    Track(TrackCommandArgs),
    #[command()]
    List(ListCommandArgs),
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut config = Config::load()?;

    let result = match cli.command {
        Command::Expand(args) => commands::expand(args, &config),
        Command::Setup(args) => commands::setup(args),
        Command::Config(args) => commands::config(args, &mut config),
        Command::Cleanup(args) => commands::cleanup(args),
        Command::New(args) => commands::new(args),
        Command::Track(args) => commands::track(args, &config),
        Command::List(args) => commands::list(&args, &config),
    };

    match result {
        Ok(_) => Ok(()),
        Err(err) => match err {
            Error::Aborted => {
                println!("Aborted!");
                exit(EXIT_STATUS_ABORTED);
            }
            Error::NotFound => {
                exit(EXIT_STATUS_NEED_CLONE);
            }
            err => Err(err),
        },
    }
}
