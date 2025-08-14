#[derive(clap::Args)]
pub struct Args {
    #[arg(
        long,
        help = "Force remote fetching of latest release by skipping the local cache"
    )]
    skip_cache: bool,
}

pub async fn handle(_args: &Args) {
    println!("Handle update");

    std::process::exit(0);
}
