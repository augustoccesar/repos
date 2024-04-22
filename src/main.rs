use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Repository name
    #[arg()]
    name: String,

    #[arg(short, long)]
    verbose: bool
}

fn main() {
    // github.com/augustoccesar/adventofcode    -- default ssh
    // augustoccesar/adventofcode               -- default github.com (overwrittable by config)
    // adventofcode                             -- default github.com + whoami (overwrittable by config)
    // aoc                                      -- configurable shortcut
    let args = Args::parse();

    println!("{}", args.name);
    println!("{}", args.verbose);
}