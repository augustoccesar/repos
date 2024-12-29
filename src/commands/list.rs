use std::{fs, path::PathBuf};

use crate::{
    config::{repos_folder_path, Config},
    Result,
};

#[derive(clap::Args)]
pub struct Args {}

pub fn list(_args: &Args, config: &Config) -> Result<()> {
    let root = Folder::root().visit(config);

    print_folder(&root, "", true);

    Ok(())
}

fn print_folder(folder: &Folder, prefix: &str, is_last: bool) {
    let folder_name = folder
        .path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    if folder.is_root {
        print!("{}", folder_name);
    } else if is_last {
        print!("{}└ {}", prefix, folder_name);
    } else {
        print!("{}├ {}", prefix, folder_name);
    }

    if let (true, Some(alias)) = (folder.is_repo, &folder.alias) {
        print!(" (alias: {})", &alias);
    }

    println!();

    for (i, subfolder) in folder.sub_folders.iter().enumerate() {
        let mut new_prefix = String::from(prefix);

        if !folder.is_root {
            if is_last {
                new_prefix.push_str("  ");
            } else {
                new_prefix.push_str("│ ");
            }
        }

        let is_last_subfolder = i == folder.sub_folders.len() - 1;
        print_folder(subfolder, &new_prefix, is_last_subfolder);
    }
}

#[derive(Debug, Default)]
struct Folder {
    path: PathBuf,
    is_root: bool,
    is_repo: bool,
    alias: Option<String>,
    sub_folders: Vec<Folder>,
}

impl Folder {
    fn new(path: PathBuf, is_repo: bool, alias: Option<String>) -> Self {
        Self {
            path,
            is_repo,
            alias,
            ..Default::default()
        }
    }

    fn root() -> Self {
        Self {
            path: PathBuf::from(&repos_folder_path()),
            is_root: true,
            ..Default::default()
        }
    }

    fn visit(mut self, config: &Config) -> Self {
        let paths = fs::read_dir(&self.path).unwrap();
        for item in paths.flatten() {
            if item.metadata().unwrap().is_dir() {
                let alias = if let Some(aliases) = &config.aliases {
                    aliases
                        .iter()
                        .find(|&(_, v)| *v == item.path().display().to_string())
                        .map(|kv| kv.0.to_owned())
                } else {
                    None
                };

                let is_repo = item.path().join(".git").exists();

                let mut folder = Folder::new(item.path(), is_repo, alias);
                if !folder.is_repo {
                    folder = folder.visit(config);
                }

                self.sub_folders.push(folder);
            }
        }

        self
    }
}
