use std::path::Path;

use anyhow::{Context, anyhow};
use gix_url::Url;

pub fn get_repo_root() -> Result<String, anyhow::Error> {
    let rev_parse_output = std::process::Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .context("failed to execute git rev-parse")?;

    if rev_parse_output.status.success() {
        Ok(String::from_utf8(rev_parse_output.stdout)
            .expect("git rev-parse stdout should be valid utf8"))
    } else {
        let error = String::from_utf8(rev_parse_output.stderr)
            .expect("git rev-parse stderr should be valid utf8");

        Err(anyhow!(error))
    }
}

pub fn clone(url: &Url, destination: &Path) -> Result<(), anyhow::Error> {
    let clone_output = std::process::Command::new("git")
        .args(["clone", &url.to_string(), &destination.to_string_lossy()])
        .output()
        .context("failed to execute git clone")?;

    if clone_output.status.success() {
        Ok(())
    } else {
        let error =
            String::from_utf8(clone_output.stderr).expect("git clone stderr should be valid utf8");

        Err(anyhow!(error))
    }
}
