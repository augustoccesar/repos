use std::{
    fs::{self, OpenOptions},
    io::{Read, Seek, Write},
    path::Path,
};

use clap::Args;

use crate::config::{rc_file_path, shell_file_path};
use crate::Result;

/// Remove what has been set by the setup command
#[derive(Args)]
pub struct CleanupCommandArgs {}

pub fn cleanup(_: CleanupCommandArgs) -> Result<()> {
    let mut rc_file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(rc_file_path())?;

    let mut rc_file_data = String::new();
    rc_file.read_to_string(&mut rc_file_data)?;

    let shell_file_path = shell_file_path();
    let lookup = format!(". {shell_file_path}");

    if let Some(start_index) = rc_file_data.find(&lookup) {
        let end_index = start_index + lookup.len();

        let file_bytes = rc_file_data.as_bytes();
        let clean_file = [&file_bytes[0..start_index], &file_bytes[end_index..]].concat();

        rc_file.set_len(0)?;
        rc_file.rewind()?;
        rc_file.write_all(&clean_file)?;
    }

    if Path::new(shell_file_path).exists() {
        fs::remove_file(shell_file_path)?;
    }

    Ok(())
}
