use anyhow::{Context, anyhow};

pub fn get_repo_root() -> Result<String, anyhow::Error> {
    let rev_parse_output = std::process::Command::new("git")
        .args(&["rev-parse", "--show-toplevel"])
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
