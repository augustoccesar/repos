use std::collections::HashMap;

use clap::{Args, Subcommand};

use crate::{config::Config, repo_name::RepoName, Result};

#[derive(Args, Debug)]
pub struct ConfigCommandArgs {
    #[command(subcommand)]
    subcommand: ConfigSubcommand,
}

#[derive(Subcommand, Debug)]
enum ConfigSubcommand {
    AddAlias(ConfigCommandAddAliasArgs),
    ListAliases,
    SetDefaultHost(ConfigCommandSetDefaultHostArgs),
    SetDefaultUsername(ConfigCommandSetDefaultUsernameArgs),
}

#[derive(Args, Debug)]
struct ConfigCommandAddAliasArgs {
    #[arg()]
    alias: String,
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
                .insert(add_alias.alias, repo_name);
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