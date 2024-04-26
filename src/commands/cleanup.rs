use std::{
    fs::OpenOptions,
    io::{Read, Seek, Write},
    process::exit,
};

use clap::Args;

use crate::config::{rc_file_path, shell_file_path};

#[derive(Args)]
pub struct CleanupCommandArgs {}

pub fn cleanup(_: CleanupCommandArgs) {
    let mut rc_file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(rc_file_path())
        .unwrap();

    let mut rc_file_data = String::new();
    rc_file.read_to_string(&mut rc_file_data).unwrap();

    let shell_file_path = shell_file_path();
    let lookup = format!(". {shell_file_path}");
    let start_index = rc_file_data.find(&lookup);
    if start_index.is_none() {
        println!("Nothing to cleanup!");
        exit(0);
    }

    let start_index = start_index.unwrap();
    let end_index = start_index + lookup.len();

    let file_bytes = rc_file_data.as_bytes();
    let clean_file = [&file_bytes[0..start_index], &file_bytes[end_index..]].concat();

    rc_file.set_len(0).unwrap();
    rc_file.rewind().unwrap();
    rc_file.write_all(&clean_file).unwrap();

    println!("Finished!");
    exit(0);
}
