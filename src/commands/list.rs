use crate::config::Config;

#[derive(clap::Args)]
pub struct Args {
    #[arg(long, short, help = "Text to look for on repositories path")]
    filter: Option<String>,
}

pub async fn handle(_args: &Args, config: &Config) {
    let repositories = crate::list_repositories(&config.base_path);

    for repository in repositories {
        println!("{}", repository.to_str().unwrap());
    }

    std::process::exit(0);
}
