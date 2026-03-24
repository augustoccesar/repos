use std::{
    fs::{self, DirEntry},
    path::{Path, PathBuf},
};

const MAX_DEPTH: u8 = 10;

pub fn list_repositories(base_path: &Path) -> Vec<PathBuf> {
    let mut repositories = Vec::<PathBuf>::new();

    for entry in fs::read_dir(base_path)
        .unwrap()
        .filter_map(|entry| entry.ok())
    {
        let Ok(entry_type) = entry.file_type() else {
            continue;
        };

        if !entry_type.is_dir() {
            continue;
        }

        walk_dir(&mut repositories, &entry, 1);
    }

    repositories
}

fn walk_dir(repositories: &mut Vec<PathBuf>, dir_entry: &DirEntry, depth: u8) {
    if depth > MAX_DEPTH {
        return;
    }

    let entries: Vec<DirEntry> = fs::read_dir(dir_entry.path())
        .unwrap()
        .filter_map(|entry| entry.ok())
        .collect();

    if entries.iter().any(|entry| entry.file_name() == ".git") {
        repositories.push(dir_entry.path());

        return;
    }

    for entry in entries {
        let Ok(entry_type) = entry.file_type() else {
            continue;
        };

        if entry_type.is_dir() {
            walk_dir(repositories, &entry, depth + 1);
        }
    }
}
