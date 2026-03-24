use std::{
    io::{self, Write},
    path::PathBuf,
};

use anyhow::{Context, anyhow};
use gix_url::Url;

use crate::{config::Config, git};

#[derive(clap::Args)]
pub struct Args {
    #[arg(
        long_help = "Name, URL or index of the repository to expand. If no value is provided, it will assume is the current working
directory.

The index of a repository can be checked on the config.toml file or by running `repos list`."
    )]
    name: Option<String>,

    /// If should clone the repo if not found locally.
    #[arg(long, default_value = "false")]
    clone: bool,
}

pub async fn handle(args: &Args, config: &Config) {
    match expand(args.name.as_deref(), config) {
        Ok(repository) => match (repository.path.exists(), args.clone) {
            (true, _) => {
                println!("{}", repository.path.to_string_lossy());
                std::process::exit(0);
            }
            (false, false) => {
                println!(
                    "Repository not found!\nLookup path: {}",
                    repository.path.to_string_lossy()
                );
                std::process::exit(1);
            }
            (false, true) => {
                println!("Repository not found locally.");
                println!(
                    "Lookup path: {}",
                    repository
                        .path
                        .to_str()
                        .expect("constructed local path should be valid unicode")
                );
                println!(
                    "Do you want to clone it from {}? (y, N)",
                    repository.clone_url.to_bstring()
                );

                loop {
                    let mut answer = String::new();
                    let _ = io::stdout().flush();
                    io::stdin()
                        .read_line(&mut answer)
                        .expect("Input should be a valid string");

                    match answer.trim().to_lowercase().as_str() {
                        "y" => break,
                        "n" => {
                            println!("Aborted!");
                            std::process::exit(1);
                        }
                        _ => println!("Invalid input"),
                    }
                }

                if let Err(error) = git::clone(&repository.clone_url, &repository.path) {
                    println!("Failed to clone repository: {error}");
                    std::process::exit(1);
                }

                println!("{}", repository.path.to_string_lossy());
                std::process::exit(0);
            }
        },
        Err(error) => {
            eprintln!("{}", error);

            std::process::exit(1);
        }
    }
}

fn expand(input: Option<&str>, config: &Config) -> Result<Repository, anyhow::Error> {
    let base_path = &config.base_path;

    let input = match input {
        Some(input) => input,
        None => {
            let base_path = base_path.to_str().expect("base path to be a valid path");
            let git_root = git::get_repo_root()?;

            if !git_root.starts_with(base_path) {
                println!("This directory/repository is not managed by repos.");
                println!("Only directories under '{base_path}' are.");

                std::process::exit(1);
            } else {
                &git_root.replace(base_path, "")
            }
        }
    };

    Repository::resolve(input, config)
}

#[derive(Debug)]
struct Repository {
    path: PathBuf,
    clone_url: Url,
}

impl Repository {
    fn resolve(input: &str, config: &Config) -> Result<Self, anyhow::Error> {
        let input = input.trim_start_matches('/');
        let url = gix_url::parse(input.into()).context("parsing repository name arg")?;

        match url.scheme {
            gix_url::Scheme::File => {
                if let Some(index) = &config.index
                    && url.path.starts_with(b"@")
                {
                    let key = std::str::from_utf8(&url.path[1..])
                        .context("failed to get index key from input")?;

                    let path = index
                        .get(key)
                        .ok_or_else(|| anyhow!("index '{}' not found", key))?;

                    return Self::resolve(path, config);
                }

                if let Some(aliases) = &config.aliases
                    && !url.path.contains(&b'/')
                {
                    if let Some(input) = aliases.get(&url.path.to_string()) {
                        return Self::resolve(input, config);
                    }
                }

                let path = url.path.to_string();
                let path_parts = path.split("/").collect::<Vec<&str>>();

                let (host, user, repo) = match path_parts.len() {
                    1 => {
                        // This will assume that the value is the repository name and will use the default
                        // host and default user to create a {default_host}/{default_user}/{repository} format.
                        (
                            config.host.as_str(),
                            config.username.as_str(),
                            path_parts[0],
                        )
                    }
                    2 => {
                        // This will assume that we have both the user and repository, and will use the default
                        // host to create a {default_host}/{user}/{repository} format.
                        (config.host.as_str(), path_parts[0], path_parts[1])
                    }
                    3 => {
                        // This will assume that we have the host, user and repository, and will
                        // create a {host}/{user}/{repository} format.
                        (path_parts[0], path_parts[1], path_parts[2])
                    }
                    _ => {
                        // TODO: Not sure about this one.
                        todo!()
                    }
                };

                let full_path = config.base_path.join(host).join(user).join(repo);
                let clone_url = Url::from_parts(
                    gix_url::Scheme::Ssh,
                    Some(user.to_owned()),
                    None,
                    Some(host.to_owned()),
                    None,
                    format!("/{repo}").into(),
                    false,
                )?;

                return Ok(Self {
                    path: full_path,
                    clone_url,
                });
            }
            gix_url::Scheme::Https
            | gix_url::Scheme::Http
            | gix_url::Scheme::Ssh
            | gix_url::Scheme::Git => {
                let path = url.path.to_string();
                let path = path.trim_start_matches('/').trim_end_matches(".git");

                let host = url
                    .host()
                    .ok_or_else(|| anyhow!("Expects git urls to have a host"))?;

                let full_path = config.base_path.join(host).join(path);

                return Ok(Self {
                    path: full_path,
                    clone_url: url,
                });
            }
            gix_url::Scheme::Ext(_) => return Err(anyhow!("Unsupported URL scheme")),
        }
    }
}

#[cfg(test)]
mod test {
    use std::{collections::HashMap, path::PathBuf};

    use crate::{commands::expand::expand, config::Config};

    #[tokio::test]
    async fn test_expand_examples() {
        let test_cases = [
            ("@1", "/test/base/github.com/rust-lang/rust"),
            (
                "git://git.kernel.org/pub/scm/bluetooth/bluez.git",
                "/test/base/git.kernel.org/pub/scm/bluetooth/bluez",
            ),
            (
                "https://git.kernel.org/pub/scm/bluetooth/bluez.git",
                "/test/base/git.kernel.org/pub/scm/bluetooth/bluez",
            ),
            (
                "https://kernel.googlesource.com/pub/scm/bluetooth/bluez.git",
                "/test/base/kernel.googlesource.com/pub/scm/bluetooth/bluez",
            ),
            (
                "git@github.com:augustoccesar/repos.git",
                "/test/base/github.com/augustoccesar/repos",
            ),
            (
                "https://github.com/augustoccesar/repos.git",
                "/test/base/github.com/augustoccesar/repos",
            ),
            (
                "github.com/augustoccesar/repos",
                "/test/base/github.com/augustoccesar/repos",
            ),
            (
                "augustoccesar/repos",
                "/test/base/github.com/augustoccesar/repos",
            ),
            ("repos", "/test/base/github.com/augustoccesar/repos"),
            ("aliased-repos", "/test/base/github.com/augustoccesar/repos"),
        ];

        for (example, expected_path) in test_cases {
            let config = Config {
                host: "github.com".to_string(),
                username: "augustoccesar".to_string(),
                base_path: PathBuf::new().join("/test").join("base"),
                index: Some(HashMap::from([(
                    "1".to_string(),
                    "github.com/rust-lang/rust".to_string(),
                )])),
                aliases: Some(HashMap::from([(
                    "aliased-repos".to_string(),
                    "github.com/augustoccesar/repos".to_string(),
                )])),
            };

            let result = expand(Some(example), &config);

            assert!(
                result.is_ok(),
                "failed to expand '{}': {}",
                example,
                result.as_ref().err().unwrap()
            );

            assert_eq!(
                result.unwrap().path.to_str().unwrap(),
                expected_path,
                "expand('{}') returned incorrect path",
                example
            );
        }
    }
}
