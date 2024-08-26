use std::path::Path;

use crate::{config::repos_folder_path, error::Error, Result};
use clap::Args;

#[derive(Args)]
pub struct NewCommandArgs {
    #[arg()]
    name: String,
}

pub fn new(args: NewCommandArgs) -> Result<()> {
    let new_project_path = Path::new(repos_folder_path())
        .join("untracked")
        .join(args.name);

    let output = std::process::Command::new("git")
        .args(["init", new_project_path.to_str().unwrap()])
        .output()?;

    if !output.status.success() {
        return Err(Error::Init(format!(
            "Failed to init repo: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
        .into());
    }

    println!("{}", new_project_path.to_str().unwrap());

    Ok(())
}
