#![warn(clippy::pedantic)]

use std::env::args;
use anyhow::Result;
use clippit::{output};
use std::process::Command;

/// Use -v to see the `cargo clippy` command and output.
fn main() -> Result<()> {
    let mut args: Vec<String> = args().skip(1).collect();
    args.insert(0, "clippy".to_string());

    let is_verbose = args.iter().any(|arg| {arg == "-v" || arg == "--verbose"});

    let mut command = Command::new("cargo");

    command.args(args);

    let clippy_output = command.output()?;
    let clippy_string = std::str::from_utf8(&*clippy_output.stderr)?;

    if is_verbose {
        eprintln!("clippy command: {:?}", command);
        eprintln!("clippy output: {clippy_string}");
    }


    output(clippy_string, &mut std::io::stderr())?;

    std::process::exit(clippy_output.status.code().unwrap_or(0));
}
