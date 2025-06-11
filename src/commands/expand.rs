use anyhow::{Context, Result};

#[derive(clap::Args)]
pub struct Args {
    #[arg(
        long_help = "Name, URL or index of the repository to expand. If no value is provided, it will assume is the current working
directory.

The index of a repository can be checked on the config.toml file or by running `repos list`."
    )]
    name: String,
}

pub async fn handle(args: &Args) -> Result<i32> {
    let url = gix_url::parse(args.name.as_str().into()).context("parsing repository name arg")?;

    if url.scheme == gix_url::Scheme::File && url.path.starts_with(b"@") {
        println!("Parsing and index");
    }

    println!("Handle expand");

    Ok(0)
}

#[cfg(test)]
mod test {
    use super::{Args, handle};

    const EXAMPLES: [&str; 9] = [
        "@1",
        "git://git.kernel.org/pub/scm/bluetooth/bluez.git",
        "https://git.kernel.org/pub/scm/bluetooth/bluez.git",
        "https://kernel.googlesource.com/pub/scm/bluetooth/bluez.git",
        "git@github.com:augustoccesar/repos.git",
        "https://github.com/augustoccesar/repos.git",
        "github.com/augustoccesar/repos",
        "augustoccesar/repos",
        "repos",
    ];

    #[tokio::test]
    async fn test_handle_examples() {
        for example in EXAMPLES {
            let result = handle(&Args {
                name: example.into(),
            })
            .await;

            assert!(result.is_ok());
        }
    }
}
