use anyhow::Result;

#[derive(clap::Args)]
pub struct Args {
    #[arg(
        long,
        help = "Force remote fetching of latest release by skipping the local cache"
    )]
    skip_cache: bool,
}

pub async fn handle(_args: &Args) -> Result<i32> {
    println!("Handle update");

    Ok(0)
}
