use anyhow::Result;

#[derive(clap::Args)]
pub struct Args {
    #[arg(
        long_help = "Name or index of the repository to expand. If no value is provided, it will assume is the current working
directory.

The index of a repository can be checked on the config.toml file or by running `repos list`.

For cases where the fields are not all present on the name, they will be resolved by:

host:
    1. What is on the `host` of the config.toml.
    2. Default to \"github.com\".

username:
    1. What is on the `username` of the config.toml.
    2. Default to 'user.name' system property.

Supported formats:
    - @{index}
    - git@{host}:{username}/{repository}.git
    - {host}/{username}/{repository}
    - {username}/{repository}
    - {repository}"
    )]
    name: String,
}

pub async fn handle(_args: &Args) -> Result<i32> {
    println!("Handle expand");

    Ok(0)
}
