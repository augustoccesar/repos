use crate::config::Config;

const FISH_SCRIPT: &str = include_str!("../../repos.fish");

#[derive(Clone, Debug, clap::ValueEnum)]
pub enum Shell {
    Fish,
}

#[derive(clap::Args)]
pub struct Args {
    shell: Shell,
}

pub async fn handle(_args: &Args, config: &Config) {
    let script = FISH_SCRIPT.replace("@EDITOR@", &config.editor);

    println!("{script}");

    std::process::exit(0);
}
