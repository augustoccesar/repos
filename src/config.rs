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
    #[serde(rename = "host")]
    pub default_host: Option<String>,
    #[serde(rename = "username")]
    pub default_user: Option<String>,
    pub aliases: Option<HashMap<String, String>>,
    pub index: Option<HashMap<String, String>>,
}

impl Config {
    pub fn load() -> Result<Self> {
        let mut path = Self::base_path()?;
        path.push("config.toml");

        let config_data =
            fs::read_to_string(&path).context(format!("failed to read file: {:?}", &path))?;

        let config: Self =
            toml::from_str(&config_data).context(format!("failed to parse file '{:?}'", &path))?;

        Ok(config)
    }

    pub fn base_path() -> Result<PathBuf> {
        let mut path = env::home_dir().context("failed to get home directory")?;
        path.push("repos");

        Ok(path)
    }
}
