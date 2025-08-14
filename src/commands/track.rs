#[derive(clap::Args)]
pub struct Args {}

pub async fn handle(_args: &Args) {
    println!("Handle track");

    std::process::exit(0);
}
