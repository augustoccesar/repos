mod commands;
mod config;
mod error;
mod repo_name;
mod shell;

use std::process::exit;

use clap::{command, Parser, Subcommand};

use commands::{
    ConfigCommandArgs, ExpandCommandArgs, ListCommandArgs, NewCommandArgs, TrackCommandArgs,
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
    Config(ConfigCommandArgs),
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
        Command::Config(args) => commands::config(args, &mut config),
        Command::New(args) => commands::new(args),
        Command::Track(args) => commands::track(args, &config),
        Command::List(args) => commands::list(&args, &mut config),
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
