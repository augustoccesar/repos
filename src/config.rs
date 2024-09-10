use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    path::Path,
    sync::OnceLock,
};

use serde::{Deserialize, Serialize};

use crate::Result;

static REPOS_PATH: OnceLock<String> = OnceLock::new();
static CONFIG_FILE_PATH: OnceLock<String> = OnceLock::new();
static HOME_PATH: OnceLock<String> = OnceLock::new();
static RC_FILE_PATH: OnceLock<String> = OnceLock::new();
static SHELL_FILE_PATH: OnceLock<String> = OnceLock::new();

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub aliases: Option<HashMap<String, String>>,
    pub default_host: Option<String>,
    pub default_username: Option<String>,
    pub home_path: Option<String>,
}

impl Config {
    pub fn load() -> Result<Self> {
        ensure_repos_folder_exists()?;

        let path = config_file_path();
        let path = Path::new(&path);

        let data = if path.exists() {
            fs::read_to_string(path)?
        } else {
            let data = "{\n}";
            let mut file = File::create(path)?;
            file.write_all(data.as_bytes())?;

            data.to_string()
        };

        let parsed_data = serde_json::from_str(&data)?;

        Ok(parsed_data)
    }

    pub fn save(&mut self) -> Result<()> {
        ensure_repos_folder_exists()?;

        let mut config_file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(config_file_path())?;

        let data = serde_json::to_string(self)?;
        config_file.write_all(data.as_bytes())?;

        Ok(())
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

    pub fn home_path(&self) -> String {
        match &self.home_path {
            Some(home_path) => home_path.clone(),
            None => home_path().clone(),
        }
    }
}

pub fn repos_folder_path() -> &'static String {
    REPOS_PATH.get_or_init(|| format!("{}/repos", home_path()))
}

fn config_file_path() -> &'static String {
    CONFIG_FILE_PATH.get_or_init(|| format!("{}/.config.json", repos_folder_path()))
}

fn home_path() -> &'static String {
    HOME_PATH.get_or_init(|| {
        dirs::home_dir()
            .expect("failed to load home directory")
            .to_str()
            .unwrap()
            .to_string()
    })
}

pub fn rc_file_path() -> &'static String {
    RC_FILE_PATH.get_or_init(|| format!("{}/.zshrc", home_path()))
}

pub fn shell_file_path() -> &'static String {
    ensure_repos_folder_exists().expect("failed to verify existence of repos folder");

    SHELL_FILE_PATH.get_or_init(|| format!("{}/.repos_shell", repos_folder_path()))
}

fn ensure_repos_folder_exists() -> Result<()> {
    if Path::new(repos_folder_path()).exists() {
        return Ok(());
    }

    fs::create_dir_all(repos_folder_path())?;

    Ok(())
}
