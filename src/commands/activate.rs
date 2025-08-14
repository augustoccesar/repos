#[derive(clap::Args)]
pub struct Args {
    shell: Shell,
}

pub async fn handle(_args: &Args) {
    println!("Handle activate");

    std::process::exit(0);
}

#[derive(Clone, Debug, clap::ValueEnum)]
pub enum Shell {
    Fish,
}
