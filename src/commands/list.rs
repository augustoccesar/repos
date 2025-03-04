use std::{collections::HashMap, fs, path::PathBuf};

use crate::{
    config::{repos_folder_path, Config},
    Result,
};

#[derive(clap::Args)]
pub struct Args {
    #[arg(short, long)]
    filter: Option<String>,
}

pub fn list(args: &Args, config: &mut Config) -> Result<()> {
    let mut current_idx = 0usize;
    let mut index_map: HashMap<usize, String> = HashMap::new();
    let mut root = Folder::root().visit(config, &mut current_idx, &mut index_map);

    config.last_list = Some(index_map);
    if let Err(error) = config.save() {
        println!("Failed to save the list indexes to config: {:?}", error);
    }

    if args.filter.is_some() {
        root.apply_filter(args.filter.as_ref().unwrap());
    }

    print_folder(&root, "", true);

    Ok(())
}

fn print_folder(folder: &Folder, prefix: &str, is_last: bool) {
    let folder_name = PathBuf::from(&folder.path)
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

    if let Some(index) = folder.index {
        line.push_str(&format!("({}) ", index));
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

// TODO(augustoccesar)[2025-03-04]: Make this into an enum structure to avoid `is_folder`.
#[derive(Debug, Default, Clone)]
struct Folder {
    index: Option<usize>,
    path: String,
    path_from_base: String,
    is_root: bool,
    is_repo: bool,
    alias: Option<String>,
    sub_folders: Vec<Folder>,
}

impl Folder {
    fn new(path: PathBuf, is_repo: bool, alias: Option<String>) -> Self {
        let path = path.display().to_string();

        let path_from_base = path
            .strip_prefix(repos_folder_path())
            .map(String::from)
            .unwrap_or(path.clone());

        Self {
            path,
            path_from_base,
            is_repo,
            alias,
            ..Default::default()
        }
    }

    fn root() -> Self {
        let path = repos_folder_path().to_string();

        let path_from_base = path
            .strip_prefix(repos_folder_path())
            .map(String::from)
            .unwrap_or(path.clone());

        Self {
            path,
            path_from_base,
            is_root: true,
            ..Default::default()
        }
    }

    fn visit(mut self, config: &mut Config, index_tracking: &mut usize, index_map: &mut HashMap<usize, String>) -> Self {
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
                if folder.is_repo {
                    folder.index = Some(*index_tracking);

                    index_map.insert(*index_tracking, folder.path.clone());

                    *index_tracking += 1;
                } else {
                    folder = folder.visit(config, index_tracking, index_map);
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
        if self.path_from_base.contains(filter) {
            return;
        }

        if !self.sub_folders.is_empty() {
            let mut new_subfolders = Vec::with_capacity(self.sub_folders.len());

            for i in 0..self.sub_folders.len() {
                let sub_folder = &mut self.sub_folders[i];

                if sub_folder.sub_folders.is_empty() {
                    if sub_folder.path_from_base.contains(filter) {
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
