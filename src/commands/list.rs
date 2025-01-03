use std::{fs, path::PathBuf};

use crate::{
    config::{repos_folder_path, Config},
    Result,
};

#[derive(clap::Args)]
pub struct Args {
    #[arg(short, long)]
    filter: Option<String>,
}

pub fn list(args: &Args, config: &Config) -> Result<()> {
    let mut root = Folder::root().visit(config);

    if args.filter.is_some() {
        root.apply_filter(args.filter.as_ref().unwrap());
    }

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

    let mut line = String::from(prefix);

    if !folder.is_root {
        if is_last {
            line.push_str("└ ");
        } else {
            line.push_str("├ ");
        }
    }

    line.push_str(&folder_name);

    if let (true, Some(alias)) = (folder.is_repo, &folder.alias) {
        line.push_str(&format!(" (alias: {})", &alias));
    }

    println!("{}", line);

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

#[derive(Debug, Default, Clone)]
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

    // TODO(augustoccesar)[2025-01-03]:
    //   This can probably be more efficient. Would it be better to do this instead?
    //     1. Get all folders and subfolders but do not build the nested folders. Just have them all flat on an array.
    //     2. Filter out any that does not match.
    //     3. Build the sub folders.
    fn apply_filter(&mut self, filter: &str) {
        if self.path.display().to_string().contains(filter) {
            return;
        }

        if !self.sub_folders.is_empty() {
            let mut new_subfolders = Vec::with_capacity(self.sub_folders.len());

            for i in 0..self.sub_folders.len() {
                let sub_folder = &mut self.sub_folders[i];

                if sub_folder.sub_folders.is_empty() {
                    if sub_folder.path.display().to_string().contains(filter) {
                        new_subfolders.push(sub_folder.clone());
                    }
                } else {
                    sub_folder.apply_filter(filter);

                    if !sub_folder.sub_folders.is_empty() {
                        new_subfolders.push(sub_folder.clone());
                    }
                }
            }

            self.sub_folders = new_subfolders;
        }
    }
}
