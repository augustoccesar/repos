use crate::config::Config;

#[derive(clap::Args)]
pub struct Args {
    #[arg(long, short, help = "Text to look for on repositories path")]
    filter: Option<String>,
}

pub async fn handle(args: &Args, config: &Config) {
    let repositories = crate::list_repositories(&config.base_path);

    for repository in repositories {
        let repository_path_str = repository.to_string_lossy();

        if let Some(filter) = &args.filter {
            if !repository_path_str.contains(filter) {
                continue;
            }
        }

        println!("{repository_path_str}");
    }

    std::process::exit(0);
}
