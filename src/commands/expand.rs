use std::{path::Path, process::exit};

use clap::Args;

use crate::{config::Config, repo_name::RepoName};

#[derive(Args)]
pub struct ExpandCommandArgs {
    /// Repository name.
    #[arg(verbatim_doc_comment)]
    name: String,
}

pub fn expand(args: ExpandCommandArgs, config: &Config) {
    let repo_name = RepoName::try_from(&args.name);

    match repo_name {
        Ok(repo_name) => {
            let path = repo_name.local_path(config);
            let exists = Path::new(&path).exists();

            if !exists {
                clone_repo(&repo_name, config);
            }

            print!("{}", &path);
            exit(0);
        }
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    }
}

fn clone_repo(repo_name: &RepoName, config: &Config) {
    let clone_url = repo_name.clone_url(config);

    let output = std::process::Command::new("git")
        .args(["clone", &clone_url, &repo_name.local_path(config)])
        .output()
        .expect("failed to execute process");

    if !output.status.success() {
        println!(
            "failed to clone repo: {}",
            String::from_utf8_lossy(output.stderr.as_slice())
        );
    }
}
