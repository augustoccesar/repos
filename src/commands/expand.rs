use anyhow::{Context, anyhow};

use crate::config::Config;

#[derive(clap::Args)]
pub struct Args {
    #[arg(
        long_help = "Name, URL or index of the repository to expand. If no value is provided, it will assume is the current working
directory.

The index of a repository can be checked on the config.toml file or by running `repos list`."
    )]
    name: String,
}

pub async fn handle(args: &Args, config: &Config) {
    match expand(&args.name, config) {
        Ok(path) => {
            println!("{}", path);

            std::process::exit(0);
        }
        Err(error) => {
            eprintln!("{}", error);

            std::process::exit(1);
        }
    }
}

fn expand(input: &str, config: &Config) -> Result<String, anyhow::Error> {
    let url = gix_url::parse(input.into()).context("parsing repository name arg")?;
    let base_path = &config.base_path;

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

                let full_path = base_path.join(path).to_string_lossy().to_string();

                return Ok(full_path);
            }

            if let Some(aliases) = &config.aliases
                && !url.path.contains(&b'/')
            {
                if let Some(input) = aliases.get(&url.path.to_string()) {
                    return expand(input, config);
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
                    return Ok(path);
                }
            };

            let full_path = base_path
                .join(host)
                .join(user)
                .join(repo)
                .to_string_lossy()
                .to_string();

            return Ok(full_path);
        }
        gix_url::Scheme::Git => {
            let path = url.path.to_string();
            let path = path.trim_start_matches('/').trim_end_matches(".git");

            let host = url
                .host()
                .ok_or_else(|| anyhow!("Expects git urls to have a host"))?;

            let full_path = base_path
                .join(host)
                .join(path)
                .to_string_lossy()
                .to_string();

            return Ok(full_path);
        }
        gix_url::Scheme::Ssh => {
            let path = url.path.to_string();
            let path = path.trim_start_matches('/').trim_end_matches(".git");

            let host = url
                .host()
                .ok_or_else(|| anyhow!("Expects git urls to have a host"))?;

            let full_path = base_path
                .join(host)
                .join(path)
                .to_string_lossy()
                .to_string();

            return Ok(full_path);
        }
        gix_url::Scheme::Http => todo!(),
        gix_url::Scheme::Https => {
            let path = url.path.to_string();
            let path = path.trim_start_matches('/').trim_end_matches(".git");

            let host = url
                .host()
                .ok_or_else(|| anyhow!("Expects git urls to have a host"))?;

            let full_path = base_path
                .join(host)
                .join(path)
                .to_string_lossy()
                .to_string();

            return Ok(full_path);
        }
        gix_url::Scheme::Ext(_) => todo!(),
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

            let result = expand(example, &config);

            assert!(
                result.is_ok(),
                "failed to expand '{}': {}",
                example,
                result.as_ref().err().unwrap()
            );

            assert_eq!(
                result.unwrap(),
                expected_path,
                "expand('{}') returned incorrect path",
                example
            );
        }
    }
}
