use std::{io, path::Path};

use clap::Args;

use crate::{config::Config, error::Error, repo_name::RepoName, Result};

/// Expand the provided name into the full local path for the repository.
/// It also prompts to clone it if is not found locally
#[derive(Args)]
pub struct ExpandCommandArgs {
    /// Repository name.
    ///
    /// For cases where the fields are not all present on the format, they will be resolved by:
    ///
    ///   host:
    ///     1. What is on the `default_host` of the config.
    ///     2. Default to "github.com".
    ///
    ///   username:
    ///     1. What is on the `default_username` of the config.
    ///     2. Default to whoami::username()
    ///
    /// Supported formats:
    ///   - @{index}
    ///     - This will expand the repo with the {index} displayed on the last `repos list`
    ///   - git@{host}:{username}/{repo}.git
    ///   - {host}/{username}/{repo}
    ///   - {username}/{repo}
    ///   - {repo}
    #[arg(verbatim_doc_comment)]
    name: String,

    /// If should clone the repo if not found locally.
    #[arg(long, default_value = "false")]
    clone: bool,
}

pub fn expand(args: ExpandCommandArgs, config: &Config) -> Result<()> {
    if args.name.starts_with("@") {
        return match &config.last_list {
            Some(last_list) => {
                let index = args.name[1..].parse::<usize>()?;
                let path = last_list.get(&index).ok_or(Error::NotFound)?;

                print!("{}", path);
                Ok(())
            }
            None => Err(Error::NotFound),
        }
    }

    let repo_name = RepoName::try_from(&args.name)?;

    let path = repo_name.local_path(config)?;
    let exists = Path::new(&path).exists();

    if !exists {
        let clone_url = repo_name.clone_url(config);

        if args.clone {
            let mut confirmation = String::new();
            println!("Repo not found locally.");
            println!("- Local path:\t{}", &path);
            println!("- Git repo:\t{}", &clone_url);
            println!("Do you want to clone it? (y/n - only 'y' continue)");

            io::stdin().read_line(&mut confirmation)?;
            if confirmation.trim() != "y" {
                return Err(Error::Aborted);
            }

            println!("Cloning repo...");
            clone_repo(&clone_url, &path)?;
        } else {
            println!("Repo not found locally in {}.", &path);
            println!("Run with --clone if want to clone it.");

            return Err(Error::NotFound);
        }
    }

    print!("{}", &path);
    Ok(())
}

fn clone_repo(clone_url: &str, target_path: &str) -> Result<()> {
    let output = std::process::Command::new("git")
        .args(["clone", clone_url, target_path])
        .output()?;

    if !output.status.success() {
        return Err(Error::Clone(format!(
            "Failed to clone repo: {}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }

    Ok(())
}
