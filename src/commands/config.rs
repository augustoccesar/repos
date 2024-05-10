use std::collections::HashMap;

use clap::{Args, Subcommand};

use crate::{config::Config, repo_name::RepoName, Result};

/// Handle the config file
#[derive(Args, Debug)]
pub struct ConfigCommandArgs {
    #[command(subcommand)]
    subcommand: ConfigSubcommand,
}

#[derive(Subcommand, Debug)]
enum ConfigSubcommand {
    /// Create an alias to a repo.
    AddAlias(ConfigCommandAddAliasArgs),
    /// List all configured aliases.
    ListAliases,
    /// Set a default host. If this is not set, it will default to "github.com"
    SetDefaultHost(ConfigCommandSetDefaultHostArgs),
    /// Set a default username. If this is not set, it will default to whoami::username()
    SetDefaultUsername(ConfigCommandSetDefaultUsernameArgs),
}

#[derive(Args, Debug)]
struct ConfigCommandAddAliasArgs {
    /// Alias to a repo. E.g. rust
    #[arg()]
    alias: String,
    /// Target of the alias. E.g. rust-lang/rust
    ///
    /// The saved target will always be the expanded version (whatever would
    /// have been returned by the 'repos expand' command).
    #[arg()]
    repo_name: String,
}

#[derive(Args, Debug)]
struct ConfigCommandSetDefaultHostArgs {
    #[arg()]
    host: String,
}

#[derive(Args, Debug)]
struct ConfigCommandSetDefaultUsernameArgs {
    #[arg()]
    username: String,
}

pub fn config(args: ConfigCommandArgs, config: &mut Config) -> Result<()> {
    match args.subcommand {
        ConfigSubcommand::AddAlias(add_alias) => {
            if config.aliases.is_none() {
                config.aliases = Some(HashMap::new());
            }

            let repo_name = RepoName::try_from(&add_alias.repo_name)?.local_path(config)?;

            config
                .aliases
                .as_mut()
                .unwrap()
                .insert(add_alias.alias.clone(), repo_name.clone());

            println!("Alias added:");
            println!("  {} => {}", add_alias.alias, repo_name);
        }
        ConfigSubcommand::ListAliases => match &config.aliases {
            Some(aliases) => {
                println!("Aliases:");
                for (alias, repo_name) in aliases {
                    println!("  {} => {}", alias, repo_name);
                }
            }
            None => {
                println!("No aliases configured.");
            }
        },
        ConfigSubcommand::SetDefaultHost(args) => {
            config.default_host = Some(args.host);
            config.save()?;
        }
        ConfigSubcommand::SetDefaultUsername(args) => {
            config.default_username = Some(args.username);
            config.save()?;
        }
    }

    config.save()?;

    Ok(())
}
