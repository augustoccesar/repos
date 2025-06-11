#![allow(dead_code)]
use std::path::PathBuf;

use anyhow::Context;

#[derive(Debug)]
pub struct Repository {
    local_path: PathBuf,
}

impl Repository {
    pub fn parse(input: &str) -> anyhow::Result<Self> {
        let url = gix_url::parse(input.into()).context("parsing repository")?;

        if url.scheme == gix_url::Scheme::File && url.path.starts_with(b"@") {
            println!("Parsing and index");
        }

        Ok(Repository {
            local_path: PathBuf::new(),
        })
    }
}

#[cfg(test)]
mod test {
    use crate::repository::Repository;

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

    #[test]
    fn test_parsing_examples() {
        for example in EXAMPLES {
            let result = Repository::parse(example);

            assert!(result.is_ok());
        }
    }
}
