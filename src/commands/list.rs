use std::{fs, path::PathBuf};

use crate::{
    config::{repos_folder_path, Config},
    Result,
};

#[derive(clap::Args)]
pub struct Args {}

pub fn list(_args: &Args, config: &Config) -> Result<()> {
    print_folder(&PathBuf::from(&repos_folder_path()), config, 0);

    Ok(())
}

pub fn print_folder(path: &PathBuf, config: &Config, level: u16) {
    let paths = fs::read_dir(path).unwrap();
    for item in paths.flatten() {
        if item.metadata().unwrap().is_dir() {
            let folder_name = item.file_name().into_string().unwrap();
            print!("{}{}", "| ".repeat(level as usize), folder_name);

            if let Some(aliases) = &config.aliases {
                if let Some(entry) = aliases
                    .iter()
                    .find(|&(_, v)| *v == item.path().display().to_string())
                {
                    print!(" (alias: {})", entry.0);
                }
            }

            println!();

            if !item.path().join(".git").exists() {
                print_folder(&item.path(), config, level + 1);
            }
        }
    }
}
