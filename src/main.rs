// TODO: Error handling. Try to remove some of the .unwraps where it makes sense.
// TODO: Replace hardcoded github.com hosts to be dynamic.
// TODO: Maybe add an unistall command as well?
// TODO: Add docs to the CLI parameters.

use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    io::{Read, Seek, Write},
    path::Path,
    process::exit,
    sync::OnceLock,
};

use clap::{command, Args, Parser, Subcommand};
use regex::Regex;
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

#[derive(Args, Debug)]
struct ConfigCommandAddAliasArgs {
    #[arg()]
    alias: String,
    #[arg()]
    repo_name: String,
}

#[derive(Args, Debug)]
struct ConfigCommandSetDefaultHostArgs {
    #[arg()]
    host: String,
}

#[derive(Args, Debug)]
struct ConfigCommandSetDefaultUsernameArgs {
    #[arg()]
    username: String,
}

#[derive(Subcommand, Debug)]
enum ConfigSubcommand {
    AddAlias(ConfigCommandAddAliasArgs),
    ListAliases,
    SetDefaultHost(ConfigCommandSetDefaultHostArgs),
    SetDefaultUsername(ConfigCommandSetDefaultUsernameArgs),
}

#[derive(Args, Debug)]
struct ConfigCommandArgs {
    #[command(subcommand)]
    subcommand: ConfigSubcommand,
}

#[derive(Subcommand)]
enum Command {
    #[command()]
    Expand(ExpandCommandArgs),
    #[command()]
    Install(InstallCommandArgs),
    #[command()]
    Config(ConfigCommandArgs),
}

#[derive(Debug)]
enum RepoName {
    CloneUrl(String),
    Full(String, String, String),
    UserRepo(String, String),
    RepoOnly(String),
}

impl RepoName {
    pub fn local_path(&self, config: &Config) -> String {
        match self {
            RepoName::CloneUrl(clone_url) => {
                let captures = clone_url_regex().captures(clone_url).unwrap();

                let host = captures.get(1).unwrap().as_str();
                let username = captures.get(2).unwrap().as_str();
                let repo = captures.get(3).unwrap().as_str();

                format!("{}/repos/{host}/{username}/{repo}", home_path(),)
            }
            RepoName::Full(host, username, repo) => {
                format!("{}/repos/{host}/{username}/{repo}", home_path(),)
            }
            RepoName::UserRepo(username, repo) => {
                let host = config.host();
                let home_path = home_path();

                format!("{home_path}/repos/{host}/{username}/{repo}")
            }
            RepoName::RepoOnly(repo_name) => {
                if let Some(aliases) = &config.aliases {
                    if let Some(alias) = aliases.get(repo_name) {
                        if alias == repo_name {
                            panic!("Infinite loop");
                        }

                        let alias_repo_name = RepoName::try_from(alias).unwrap();

                        return alias_repo_name.local_path(config);
                    }
                }

                todo!()
            }
        }
    }

    #[allow(dead_code)]
    pub fn clone_url(&self, config: &Config) -> String {
        match self {
            RepoName::CloneUrl(clone_url) => clone_url.clone(),
            RepoName::Full(host, username, repo) => {
                format!("git@{host}:{username}/{repo}.git")
            }
            RepoName::UserRepo(username, repo) => {
                let host = config.host();

                format!("git@{host}:{username}/{repo}.git")
            }
            RepoName::RepoOnly(_) => todo!(),
        }
    }
}

impl TryFrom<&String> for RepoName {
    type Error = String; // TODO: Better error type

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        if clone_url_regex().is_match(value) {
            return Ok(RepoName::CloneUrl(value.clone()));
        }

        let parts: Vec<&str> = value.split('/').collect();

        match parts.len() {
            3 => {
                let host = parts.get(0).unwrap().to_string(); // TODO: proper validate format of host.
                let username = parts.get(1).unwrap().to_string();
                let repo = parts.get(2).unwrap().to_string();

                Ok(RepoName::Full(host, username, repo))
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
    default_host: Option<String>,
    default_username: Option<String>,
}

impl Config {
    pub fn load() -> Self {
        ensure_repos_folder_exists();

        let path = config_file_path();
        let path = Path::new(&path);

        let data = if path.exists() {
            fs::read_to_string(path).unwrap()
        } else {
            let data = "{\n}";
            let mut file = File::create(path).unwrap();
            file.write_all(data.as_bytes()).unwrap();

            data.to_string()
        };

        serde_json::from_str(&data).unwrap()
    }

    pub fn save(&mut self) {
        ensure_repos_folder_exists();

        let mut config_file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(config_file_path())
            .expect("failed to open config file");

        let data = serde_json::to_string(self).expect("failed to serialize config as json");
        config_file
            .write_all(data.as_bytes())
            .expect("failed to write config file");
    }

    pub fn host(&self) -> String {
        match &self.default_host {
            Some(host) => host.clone(),
            None => String::from("github.com"),
        }
    }

    pub fn username(&self) -> String {
        match &self.default_username {
            Some(username) => username.clone(),
            None => whoami::username(),
        }
    }
}

fn repos_folder_path() -> &'static String {
    static REPOS_PATH: OnceLock<String> = OnceLock::new();

    REPOS_PATH.get_or_init(|| format!("{}/repos", home_path()))
}

fn config_file_path() -> &'static String {
    static CONFIG_FILE_PATH: OnceLock<String> = OnceLock::new();

    CONFIG_FILE_PATH.get_or_init(|| format!("{}/.config.json", repos_folder_path()))
}

fn home_path() -> &'static String {
    static HOME_PATH: OnceLock<String> = OnceLock::new();

    HOME_PATH.get_or_init(|| {
        dirs::home_dir()
            .expect("failed to load home directory")
            .to_str()
            .unwrap()
            .to_string()
    })
}

fn clone_url_regex() -> &'static Regex {
    static CLONE_URL_REGEX: OnceLock<Regex> = OnceLock::new();

    CLONE_URL_REGEX
        .get_or_init(|| Regex::new(r"git@(.+):(.+)\/(.+)\.git").expect("failed to compile regex"))
}

fn ensure_repos_folder_exists() {
    if Path::new(repos_folder_path()).exists() {
        return;
    }

    fs::create_dir_all(repos_folder_path()).unwrap();
}

// TODO: Can this dependency on Config be removed?
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

fn main() {
    //   - git@github.com:augustoccesar/adventofcode.git    -- as is. /git@(.+):(.+)\/(.+)\.git/
    //   - github.com/augustoccesar/adventofcode            -- default ssh
    //   - augustoccesar/adventofcode                       -- default github.com (overwrittable by config)
    //   - adventofcode                                     -- default github.com + whoami (overwrittable by config)
    //   - aoc                                              -- configurable shortcut
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
                    } else {
                        clone_repo(&repo_name, &config);
                        print!("{}", &path);
                    }

                    exit(0);
                }
                Err(_) => todo!(),
            }
        }
        Command::Install(_) => {
            // TODO: Can format this nicer?
            let function = "
###begin:repos_functions
# repos expand will return the path of the repo locally and this 
# function will only be responsible for cd'ing into the folder if successful
# or print the output if it fails.
function rcd() {
    OUTPUT=$(repos expand $1)
    if [[ $? -eq 0 ]]; then
        cd $OUTPUT
    else
        echo $OUTPUT
    fi
}
###end:repos_functions
            ";

            // TODO: Support files other than zsh
            let rc_file_path = format!("{}/.zshrc", home_path());

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
        Command::Config(args) => {
            let mut config = Config::load();

            match args.subcommand {
                ConfigSubcommand::AddAlias(add_alias) => {
                    if let None = config.aliases {
                        config.aliases = Some(HashMap::new());
                    }

                    config
                        .aliases
                        .as_mut()
                        .unwrap()
                        .insert(add_alias.alias, add_alias.repo_name);
                }
                ConfigSubcommand::ListAliases => match &config.aliases {
                    Some(aliases) => {
                        println!("Aliases:");
                        for (alias, repo_name) in aliases {
                            println!("  {} => {}", alias, repo_name);
                        }
                    }
                    None => {
                        println!("No aliases configured.");
                    }
                },
                ConfigSubcommand::SetDefaultHost(args) => {
                    config.default_host = Some(args.host);
                    config.save();
                }
                ConfigSubcommand::SetDefaultUsername(args) => {
                    config.default_username = Some(args.username);
                    config.save();
                }
            }

            config.save();
        }
    }
}
