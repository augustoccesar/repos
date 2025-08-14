#[derive(clap::Args)]
pub struct Args {
    #[arg(long, short, help = "Text to look for on repositories path")]
    filter: Option<String>,
}

pub async fn handle(_args: &Args) {
    println!("Handle list");

    std::process::exit(0);
}
