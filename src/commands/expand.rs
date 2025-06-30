use anyhow::{Context, Result, anyhow};

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

pub async fn handle(args: &Args, config: &Config) -> Result<i32> {
    let url = gix_url::parse(args.name.as_str().into()).context("parsing repository name arg")?;

    if url.scheme == gix_url::Scheme::File {
        if let Some(index) = &config.index
            && url.path.starts_with(b"@")
        {
            let key = std::str::from_utf8(&url.path[1..])
                .context("failed to get index key from input")?;

            let path = index
                .get(key)
                .ok_or_else(|| anyhow!("index '{}' not found", key))?;

            println!(
                "{}/{}",
                Config::base_path()
                    .context("failed to get the base path")?
                    .to_string_lossy(),
                path
            );

            return Ok(0);
        }

        if let Some(aliases) = &config.aliases
            && !url.path.contains(&b'/')
        {
            if let Some(path) = aliases.get(&url.path.to_string()) {
                println!("{}", path);

                return Ok(0);
            }
        }
    }

    Ok(0)
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::config::Config;

    use super::{Args, handle};

    const EXAMPLES: [&str; 10] = [
        "@1",
        "git://git.kernel.org/pub/scm/bluetooth/bluez.git",
        "https://git.kernel.org/pub/scm/bluetooth/bluez.git",
        "https://kernel.googlesource.com/pub/scm/bluetooth/bluez.git",
        "git@github.com:augustoccesar/repos.git",
        "https://github.com/augustoccesar/repos.git",
        "github.com/augustoccesar/repos",
        "augustoccesar/repos",
        "repos",
        "aliased-repos",
    ];

    #[tokio::test]
    async fn test_handle_examples() {
        for example in EXAMPLES {
            let config = Config {
                index: Some(HashMap::from([(
                    "1".to_string(),
                    "github.com/rust-lang/rust".to_string(),
                )])),
                aliases: Some(HashMap::from([(
                    "aliased-repos".to_string(),
                    "github.com/augustoccesar/repos".to_string(),
                )])),
                ..Default::default()
            };

            let result = handle(
                &Args {
                    name: example.into(),
                },
                &config,
            )
            .await;

            assert!(result.is_ok());
        }
    }
}
