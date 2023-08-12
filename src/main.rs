#![warn(clippy::pedantic)]

use std::env::args;
use anyhow::Result;
use clippit::{output};
use std::process::Command;

fn main() -> Result<()> {
    let is_verbose = args().any(|arg| {arg == "-v" || arg == "--v"});

    let mut command = Command::new("cargo");
    command.args(args());

    let clippy_output = command.output()?;
    let clippy_string = std::str::from_utf8(&*clippy_output.stderr)?;

    if is_verbose {
        println!("clippy command: {:?}", command);
        println!("clippy output: {clippy_string}");
    }


    output(clippy_string, &mut std::io::stderr())?;

    Ok(())
}
