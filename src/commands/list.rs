use std::collections::HashMap;

use crate::config::Config;

#[derive(clap::Args)]
pub struct Args {
    #[arg(long, short, help = "Text to look for on repositories path")]
    filter: Option<String>,
}

pub async fn handle(args: &Args, config: &Config) {
    let mut repositories = crate::list_repositories(&config.base_path);
    repositories.sort();

    let mut index = HashMap::<String, String>::new();
    for (i, repository) in repositories.iter().enumerate() {
        let repository_path_str = repository.to_string_lossy();

        index.insert(i.to_string(), repository_path_str.to_string());

        if let Some(filter) = &args.filter {
            if !repository_path_str.contains(filter) {
                continue;
            }
        }

        println!("[{i}] {repository_path_str}");
    }

    let mut new_cfg = config.clone();
    new_cfg.index = Some(index);
    // TODO(augustoccesar)[2026-04-20]: Handle error and log instead of panicking
    new_cfg.save().expect("should be able to save config file");

    std::process::exit(0);
}
