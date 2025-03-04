use std::sync::OnceLock;

use regex::Regex;

use crate::{config::Config, Error, Result};

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

                        Ok(format!(
                            "{}/repos/{host}/{username}/{repo}",
                            config.home_path()
                        ))
                    }
                    None => Err(Error::Format("Invalid clone url format".into())),
                }
            }
            RepoName::Full(host, username, repo) => Ok(format!(
                "{}/repos/{host}/{username}/{repo}",
                config.home_path()
            )),
            RepoName::UserRepo(username, repo) => {
                let host = config.host();
                let home_path = config.home_path();

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
                let home_path = config.home_path();

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
            _ => Err(Error::Format("Invalid repo name format.".into())),
        }
    }
}

fn clone_url_regex() -> &'static Regex {
    static CLONE_URL_REGEX: OnceLock<Regex> = OnceLock::new();

    CLONE_URL_REGEX
        .get_or_init(|| Regex::new(r"git@(.+):(.+)\/(.+)\.git").expect("failed to compile regex"))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{config::Config, repo_name::RepoName};

    #[test]
    fn local_path_from_clone_url_works() {
        let config = Config {
            aliases: Some(HashMap::new()),
            default_host: Some(String::from("github.com")),
            default_username: Some(String::from("username")),
            home_path: Some(String::from("/Users/username")),
            last_list: None,
        };
        let clone_url = "git@github.com:rust-lang/rust.git";

        let result = RepoName::CloneUrl(clone_url.into())
            .local_path(&config)
            .unwrap();

        assert_eq!("/Users/username/repos/github.com/rust-lang/rust", result)
    }

    #[test]
    fn local_path_from_full_works() {
        let config = Config {
            aliases: Some(HashMap::new()),
            default_host: Some(String::from("github.com")),
            default_username: Some(String::from("username")),
            home_path: Some(String::from("/Users/username")),
            last_list: None,
        };

        let result = RepoName::Full("github.com".into(), "rust-lang".into(), "rust".into())
            .local_path(&config)
            .unwrap();

        assert_eq!("/Users/username/repos/github.com/rust-lang/rust", result)
    }

    #[test]
    fn local_path_from_user_repo_works() {
        let config = Config {
            aliases: Some(HashMap::new()),
            default_host: Some(String::from("github.com")),
            default_username: Some(String::from("username")),
            home_path: Some(String::from("/Users/username")),
            last_list: None,
        };

        let result = RepoName::UserRepo("rust-lang".into(), "rust".into())
            .local_path(&config)
            .unwrap();

        assert_eq!("/Users/username/repos/github.com/rust-lang/rust", result)
    }

    #[test]
    fn local_path_from_repo_only_works() {
        let config = Config {
            aliases: Some(HashMap::new()),
            default_host: Some(String::from("github.com")),
            default_username: Some(String::from("augustoccesar")),
            home_path: Some(String::from("/Users/username")),
            last_list: None,
        };

        let result = RepoName::RepoOnly("repos".into())
            .local_path(&config)
            .unwrap();

        assert_eq!(
            "/Users/username/repos/github.com/augustoccesar/repos",
            result
        )
    }

    #[test]
    fn local_path_from_repo_only_with_alias_works() {
        let mut aliases = HashMap::new();
        aliases.insert(
            String::from("r"),
            String::from("/Users/username/repos/github.com/augustoccesar/repos"),
        );

        let config = Config {
            aliases: Some(aliases),
            default_host: Some(String::from("github.com")),
            default_username: Some(String::from("username")),
            home_path: Some(String::from("/Users/username")),
            last_list: None,
        };

        let result = RepoName::RepoOnly("r".into()).local_path(&config).unwrap();

        assert_eq!(
            "/Users/username/repos/github.com/augustoccesar/repos",
            result
        )
    }
}
