use anyhow::Result;

#[derive(clap::Args)]
pub struct Args {}

pub async fn handle(_args: &Args) -> Result<i32> {
    println!("Handle track");

    Ok(0)
}
