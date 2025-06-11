use anyhow::Result;

#[derive(clap::Args)]
pub struct Args {
    shell: Shell,
}

pub async fn handle(_args: &Args) -> Result<i32> {
    println!("Handle activate");

    Ok(0)
}

#[derive(Clone, Debug, clap::ValueEnum)]
pub enum Shell {
    Fish,
}
