use std::path::Path;

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
        clone_repo(&repo_name, config)?;
    }

    print!("{}", &path);
    Ok(())
}

fn clone_repo(repo_name: &RepoName, config: &Config) -> Result<()> {
    let clone_url = repo_name.clone_url(config);

    let output = std::process::Command::new("git")
        .args(["clone", &clone_url, &repo_name.local_path(config)?])
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
