use std::sync::OnceLock;

use regex::Regex;

use crate::{
    config::{home_path, Config},
    Error, Result,
};

#[derive(Debug)]
pub enum RepoName {
    CloneUrl(String),
    Full(String, String, String),
    UserRepo(String, String),
    RepoOnly(String),
}

impl RepoName {
    pub fn local_path(&self, config: &Config) -> Result<String> {
        match self {
            RepoName::CloneUrl(clone_url) => {
                let captures = clone_url_regex().captures(clone_url);

                match captures {
                    Some(captures) => {
                        let (_, [host, username, repo]) = captures.extract();

                        Ok(format!("{}/repos/{host}/{username}/{repo}", home_path()))
                    }
                    None => Err("Invalid clone url format".into()),
                }
            }
            RepoName::Full(host, username, repo) => {
                Ok(format!("{}/repos/{host}/{username}/{repo}", home_path()))
            }
            RepoName::UserRepo(username, repo) => {
                let host = config.host();
                let home_path = home_path();

                Ok(format!("{home_path}/repos/{host}/{username}/{repo}"))
            }
            RepoName::RepoOnly(repo) => {
                if let Some(aliases) = &config.aliases {
                    if let Some(alias) = aliases.get(repo) {
                        return Ok(alias.clone());
                    }
                }

                let host = config.host();
                let username = config.username();
                let home_path = home_path();

                Ok(format!("{home_path}/repos/{host}/{username}/{repo}"))
            }
        }
    }

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
            RepoName::RepoOnly(repo) => {
                let host = config.host();
                let username = config.username();

                format!("git@{host}:{username}/{repo}.git")
            }
        }
    }
}

impl TryFrom<&String> for RepoName {
    type Error = Error;

    fn try_from(value: &String) -> std::result::Result<Self, Self::Error> {
        if clone_url_regex().is_match(value) {
            return Ok(RepoName::CloneUrl(value.clone()));
        }

        let parts: Vec<&str> = value.split('/').collect();

        match parts.len() {
            3 => {
                let host = parts.first().unwrap().to_string();
                let username = parts.get(1).unwrap().to_string();
                let repo = parts.get(2).unwrap().to_string();

                Ok(RepoName::Full(host, username, repo))
            }
            2 => Ok(RepoName::UserRepo(
                parts.first().unwrap().to_string(),
                parts.get(1).unwrap().to_string(),
            )),
            1 => Ok(RepoName::RepoOnly(parts.first().unwrap().to_string())),
            _ => Err("Invalid repo name format.".into()),
        }
    }
}

fn clone_url_regex() -> &'static Regex {
    static CLONE_URL_REGEX: OnceLock<Regex> = OnceLock::new();

    CLONE_URL_REGEX
        .get_or_init(|| Regex::new(r"git@(.+):(.+)\/(.+)\.git").expect("failed to compile regex"))
}
