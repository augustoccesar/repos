use std::{
    fs::OpenOptions,
    io::{self, Read, Seek, Write},
    process::exit,
};

use clap::Args;

use crate::config::home_path;

#[derive(Args)]
pub struct SetupCommandArgs {}

pub fn setup(_: SetupCommandArgs) {
    let shell_setup = include_str!("../../shell_setup");
    let rc_file_name = ".zshrc";

    let mut confirmation = String::new();
    println!(
        "Setup will add the following to to your {} file",
        rc_file_name
    );
    println!("```");
    println!("{}", shell_setup);
    println!("```");
    println!("Do you want to continue? (y/n - only 'y' continue)");
    match io::stdin().read_line(&mut confirmation) {
        Ok(_) => {
            if confirmation.trim() != "y" {
                println!("Aborted!");
                exit(0);
            }
        }
        Err(_) => exit(1),
    }

    let rc_file_path = format!("{}/{}", home_path(), rc_file_name);
    let mut rc_file = OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .open(rc_file_path)
        .unwrap();

    let mut rc_file_data = String::new();
    rc_file.read_to_string(&mut rc_file_data).unwrap();

    if rc_file_data.contains("###begin:repos_functions") {
        println!("Already setup!");
        exit(0);
    }

    rc_file.rewind().unwrap();
    rc_file.write_all(shell_setup.as_bytes()).unwrap();

    println!("Ready!");
    println!("Run source your '{}' to reflect changes.", rc_file_name);
    exit(0);
}
