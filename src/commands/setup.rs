use std::{
    fs::{File, OpenOptions},
    io::{self, Read, Write},
    path::Path,
};

use clap::Args;

use crate::{config::{rc_file_path, shell_file_path}, error::Error};
use crate::Result;

/// Setup helpers to make the use of repos easier. Namelly it adds shell script to make it
#[derive(Args)]
pub struct SetupCommandArgs {}

pub fn setup(_: SetupCommandArgs) -> Result<()> {
    let shell_file_data = include_str!("../../shell_setup");

    let rc_file_path = rc_file_path();
    let shell_file_path = shell_file_path();

    if !Path::new(shell_file_path).exists() {
        let mut shell_file = File::create(shell_file_path)?;
        shell_file.write_all(shell_file_data.as_bytes())?;
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

    io::stdin().read_line(&mut confirmation)?;
    if confirmation.trim() != "y" {
        return Err(Error::Aborted);
    }

    let mut rc_file = OpenOptions::new()
        .read(true)
        .append(true)
        .open(rc_file_path)?;

    let mut rc_file_data = String::new();
    rc_file.read_to_string(&mut rc_file_data)?;

    if rc_file_data.contains(&shell_setup) {
        println!("Already setup!");
        return Ok(());
    }

    rc_file.write_all(shell_setup.as_bytes())?;

    println!("Ready!");
    println!("Run 'source {}' to reflect changes.", rc_file_path);

    Ok(())
}
