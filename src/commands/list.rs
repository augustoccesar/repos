use anyhow::Result;

#[derive(clap::Args)]
pub struct Args {
    #[arg(long, short, help = "Text to look for on repositories path")]
    filter: Option<String>,
}

pub async fn handle(_args: &Args) -> Result<i32> {
    println!("Handle list");

    Ok(0)
}
