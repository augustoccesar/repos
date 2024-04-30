use std::{io, path::Path};

use clap::Args;

use crate::{config::Config, repo_name::RepoName, Result};

#[derive(Args)]
pub struct ExpandCommandArgs {
    /// Repository name.
    #[arg(verbatim_doc_comment)]
    name: String,
}

pub fn expand(args: ExpandCommandArgs, config: &Config) -> Result<()> {
    let repo_name = RepoName::try_from(&args.name)?;

    let path = repo_name.local_path(config)?;
    let exists = Path::new(&path).exists();

    if !exists {
        let clone_url = repo_name.clone_url(config);

        let mut confirmation = String::new();
        println!("Repo not found locally.");
        println!("- Local path:\t{}", &path);
        println!("- Git repo:\t{}", &clone_url);
        println!("Do you want to clone it? (y/n - only 'y' continue)");

        io::stdin().read_line(&mut confirmation)?;
        if confirmation.trim() != "y" {
            println!("Aborted!");
            return Ok(());
        }

        println!("Cloning repo...");
        clone_repo(&clone_url, &path)?;
    }

    print!("{}", &path);
    Ok(())
}

fn clone_repo(clone_url: &str, target_path: &str) -> Result<()> {
    let output = std::process::Command::new("git")
        .args(["clone", clone_url, target_path])
        .output()?;

    if !output.status.success() {
        return Err(format!(
            "Failed to clone repo: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    Ok(())
}
