use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    io::{Read, Seek, Write},
    path::Path,
    process::exit,
};

use clap::{command, Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};

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

#[derive(Debug)]
enum RepoName {
    Full(FullRepoName),
    UserRepo(String, String),
    RepoOnly(String),
}

impl RepoName {
    pub fn local_path(&self, config: &Config) -> String {
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
            RepoName::RepoOnly(repo_name) => {
                if let Some(aliases) = &config.aliases {
                    if let Some(alias) = aliases.get(repo_name) {
                        if alias == repo_name {
                            panic!("Infinite loop");
                        }

                        // TODO: Handle error
                        let alias_repo_name = RepoName::try_from(alias).unwrap();

                        return alias_repo_name.local_path(config);
                    }
                }

                todo!()
            }
        }
    }

    #[allow(dead_code)]
    pub fn clone_url(&self) -> String {
        match self {
            RepoName::Full(info) => {
                format!("git@{}:{}/{}.git", info.host, info.username, info.repo)
            }
            RepoName::UserRepo(_, _) => todo!(),
            RepoName::RepoOnly(_) => todo!(),
        }
    }
}

impl TryFrom<&String> for RepoName {
    type Error = String; // TODO: Better error type

    fn try_from(value: &String) -> Result<Self, Self::Error> {
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

// TODO: Create subcommand to manage this config
#[derive(Serialize, Deserialize)]
struct Config {
    aliases: Option<HashMap<String, String>>,
}

impl Config {
    pub fn load() -> Self {
        let folder_path = repos_folder_path();
        let path = format!("{}/.config.json", &folder_path);
        let path = Path::new(&path);

        let data = if path.exists() {
            fs::read_to_string(path).unwrap()
        } else {
            fs::create_dir_all(&folder_path).unwrap();

            let data = "{\n}";
            let mut file = File::create(path).unwrap();
            file.write_all(data.as_bytes()).unwrap();

            data.to_string()
        };

        serde_json::from_str(&data).unwrap()
    }
}

// TODO: Maybe make this into a lazy static?
fn repos_folder_path() -> String {
    let home_path = dirs::home_dir().unwrap().to_str().unwrap().to_string();
    format!("{}/repos", home_path)
}

fn main() {
    //   - github.com/augustoccesar/adventofcode    -- default ssh
    //   - augustoccesar/adventofcode               -- default github.com (overwrittable by config)
    //   - adventofcode                             -- default github.com + whoami (overwrittable by config)
    //   - aoc                                      -- configurable shortcut
    let cli = Cli::parse();
    let config = Config::load();

    match cli.command {
        Command::Expand(args) => {
            let repo_name = RepoName::try_from(&args.name);

            match repo_name {
                Ok(repo_name) => {
                    let path = repo_name.local_path(&config);
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
        Command::Install(_) => {
            // TODO: Maybe create an uninstall command as well?

            // TODO: Can format this nicer?
            let function = "
###begin:repos_functions
# repos expand will return the path of the repo locally and this 
# function will only be responsible for cd'ing into the folder if successful
# or print the output if it fails.
function r() {
    OUTPUT=$(repos expand $1)
    if [[ $? -eq 0 ]]; then
        cd $OUTPUT
    else
        echo $OUTPUT
    fi
}
###end:repos_functions
            ";

            let home_path = dirs::home_dir().unwrap().to_str().unwrap().to_string();
            // TODO: Support files other than zsh
            let rc_file_path = format!("{}/.zshrc", &home_path);

            let mut rc_file = OpenOptions::new()
                .read(true)
                .write(true)
                .append(true)
                .open(rc_file_path)
                .unwrap();

            let mut rc_file_data = String::new();
            rc_file.read_to_string(&mut rc_file_data).unwrap();

            if let Some(_) = rc_file_data.find("###begin:repos_functions") {
                println!("Already installed!");
                exit(0);
            }

            rc_file.rewind().unwrap();
            rc_file.write_all(function.as_bytes()).unwrap();

            println!("Installed!");
            println!("Run 'source ~/.zshrc' to reflect changes.");
            exit(0);
        }
    }
}
