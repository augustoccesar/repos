use std::{
    collections::HashMap,
    env,
    fs::{self},
    path::PathBuf,
};

use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    #[serde(rename = "host", default = "default_host")]
    pub host: String,
    #[serde(rename = "username", default = "default_user")]
    pub username: String,
    #[serde(rename = "base_path", default = "default_base_path")]
    pub base_path: PathBuf,
    pub aliases: Option<HashMap<String, String>>,
    pub index: Option<HashMap<String, String>>,
}

impl Config {
    pub fn load() -> Result<Self> {
        let path = config_dir_path().join("config.toml");

        let config_data =
            fs::read_to_string(&path).context(format!("failed to read file: {:?}", &path))?;

        let config: Self =
            toml::from_str(&config_data).context(format!("failed to parse file '{:?}'", &path))?;

        Ok(config)
    }
}

fn config_dir_path() -> PathBuf {
    env::home_dir().unwrap().join(".config").join("repos")
}

fn default_base_path() -> PathBuf {
    env::home_dir().unwrap().join("repos")
}

fn default_host() -> String {
    String::from("github.com")
}

fn default_user() -> String {
    whoami::username()
}
