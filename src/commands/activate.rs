const FISH_SCRIPT: &str = include_str!("../../repos.fish");

#[derive(clap::Args)]
pub struct Args {
    shell: Shell,
}

pub async fn handle(_args: &Args) {
    println!("{}", FISH_SCRIPT);

    std::process::exit(0);
}

#[derive(Clone, Debug, clap::ValueEnum)]
pub enum Shell {
    Fish,
}
