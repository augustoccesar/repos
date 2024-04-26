use std::{
    fs::{File, OpenOptions},
    io::{self, Read, Write},
    path::Path,
    process::exit,
};

use clap::Args;

use crate::config::{rc_file_path, shell_file_path};

#[derive(Args)]
pub struct SetupCommandArgs {}

pub fn setup(_: SetupCommandArgs) {
    let shell_file_data = include_str!("../../shell_setup");

    let rc_file_path = rc_file_path();
    let shell_file_path = shell_file_path();

    if !Path::new(shell_file_path).exists() {
        let mut shell_file = File::create(shell_file_path).unwrap();
        shell_file.write_all(shell_file_data.as_bytes()).unwrap();
    }

    let shell_setup = format!("\n. {shell_file_path}\n");

    let mut confirmation = String::new();
    println!(
        "Setup will add the following to to your {} file",
        rc_file_path
    );
    println!("```");
    println!("{}", &shell_setup);
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

    let mut rc_file = OpenOptions::new()
        .read(true)
        .append(true)
        .open(rc_file_path)
        .unwrap();

    let mut rc_file_data = String::new();
    rc_file.read_to_string(&mut rc_file_data).unwrap();

    if rc_file_data.contains(&shell_setup) {
        println!("Already setup!");
        exit(0);
    }

    rc_file.write_all(shell_setup.as_bytes()).unwrap();

    println!("Ready!");
    println!("Run source your '{}' to reflect changes.", rc_file_path);
    exit(0);
}
