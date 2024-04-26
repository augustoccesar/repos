use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    path::Path,
    sync::OnceLock,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub aliases: Option<HashMap<String, String>>,
    pub default_host: Option<String>,
    pub default_username: Option<String>,
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

pub fn home_path() -> &'static String {
    static HOME_PATH: OnceLock<String> = OnceLock::new();

    HOME_PATH.get_or_init(|| {
        dirs::home_dir()
            .expect("failed to load home directory")
            .to_str()
            .unwrap()
            .to_string()
    })
}

pub fn rc_file_path() -> &'static String {
    static RC_FILE_PATH: OnceLock<String> = OnceLock::new();

    RC_FILE_PATH.get_or_init(|| format!("{}/.zshrc", home_path()))
}

pub fn shell_file_path() -> &'static String {
    ensure_repos_folder_exists();

    static SHELL_FILE_PATH: OnceLock<String> = OnceLock::new();

    SHELL_FILE_PATH.get_or_init(|| format!("{}/.repos_shell", repos_folder_path()))
}

fn ensure_repos_folder_exists() {
    if Path::new(repos_folder_path()).exists() {
        return;
    }

    fs::create_dir_all(repos_folder_path()).unwrap();
}
