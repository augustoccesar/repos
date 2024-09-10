use std::env::current_dir;

use crate::{config::Config, error::Error, repo_name::RepoName, Result};
use clap::Args;

#[derive(Args)]
pub struct TrackCommandArgs {}

pub fn track(_: TrackCommandArgs, config: &Config) -> Result<()> {
    let output = std::process::Command::new("git")
        .args(["remote", "get-url", "origin"])
        .output()?;

    if !output.status.success() {
        return Err(Error::Track(format!(
            "Failed to get remote url of the repo: {}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }

    let current_dir = current_dir()?.to_str().unwrap().to_string();

    let remote_path = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let repo_name = RepoName::try_from(&remote_path)?;
    let new_dir = repo_name.local_path(config)?;

    if current_dir == new_dir {
        println!("repository already on the expected directory.");

        return Ok(());
    }

    let output = std::process::Command::new("mv")
        .args([&current_dir, &new_dir])
        .output()?;

    if !output.status.success() {
        return Err(Error::Track(format!(
            "Failed move repo to new directory: {}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }

    println!("{}", new_dir);

    Ok(())
}
