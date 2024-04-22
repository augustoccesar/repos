use std::{path::Path, process::exit};

use clap::{command, Args, Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Args)]
struct ExpandCommandArgs {
    /// Repository name.
    #[arg(verbatim_doc_comment)]
    name: String,
}

#[derive(Args)]
struct InstallCommandArgs {}

#[derive(Subcommand)]
enum Command {
    #[command()]
    Expand(ExpandCommandArgs),
    #[command()]
    Install(InstallCommandArgs),
}

#[derive(Debug)]
struct FullRepoName {
    host: String,
    username: String,
    repo: String,
}

impl FullRepoName {
    fn clone_url(&self) -> String {
        format!("git@{}:{}/{}.git", self.host, self.username, self.repo)
    }
}

#[derive(Debug)]
enum RepoName {
    Full(FullRepoName),
    UserRepo(String, String),
    RepoOnly(String),
}

impl RepoName {
    pub fn local_path(&self) -> String {
        match self {
            RepoName::Full(repo_name) => {
                format!(
                    "{}/repos/{}/{}/{}",
                    dirs::home_dir().unwrap().to_str().unwrap(),
                    repo_name.host,
                    repo_name.username,
                    repo_name.repo
                )
            }
            RepoName::UserRepo(_, _) => todo!(),
            RepoName::RepoOnly(_) => todo!(),
        }
    }

    pub fn clone_url(&self) -> String {
        todo!()
    }
}

impl TryFrom<String> for RepoName {
    type Error = String; // TODO: Better error type

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split('/').collect();

        match parts.len() {
            3 => {
                let host = parts.get(0).unwrap().to_string(); // TODO: proper validate format of host.
                let username = parts.get(1).unwrap().to_string();
                let repo = parts.get(2).unwrap().to_string();

                Ok(RepoName::Full(FullRepoName {
                    host,
                    username,
                    repo,
                }))
            }
            2 => Ok(RepoName::UserRepo(
                parts.get(0).unwrap().to_string(),
                parts.get(1).unwrap().to_string(),
            )),
            1 => Ok(RepoName::RepoOnly(parts.get(0).unwrap().to_string())),
            _ => Err(String::from("Invalid repo name format.")),
        }
    }
}

fn main() {
    //   - github.com/augustoccesar/adventofcode    -- default ssh
    //   - augustoccesar/adventofcode               -- default github.com (overwrittable by config)
    //   - adventofcode                             -- default github.com + whoami (overwrittable by config)
    //   - aoc                                      -- configurable shortcut
    let cli = Cli::parse();

    match cli.command {
        Command::Expand(args) => {
            let repo_name = RepoName::try_from(args.name);

            match repo_name {
                Ok(repo_name) => {
                    let path = repo_name.local_path();
                    let exists = Path::new(&path).exists();

                    if exists {
                        print!("{}", &path);
                        exit(0);
                    } else {
                        // TODO: clone if does not exist
                        print!("Could not find repository locally. Does not support clone yet.");
                        exit(1);
                    }
                }
                Err(_) => todo!(),
            }
        }
        Command::Install(_) => todo!(),
    }
}
