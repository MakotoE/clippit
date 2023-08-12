use std::env::args;
use anyhow::Result;
use clippit::{output};
use std::process::Command;

fn main() -> Result<()> {
    let is_verbose = args().any(|arg| {arg == "-v" || arg == "--v"});

    let mut command = Command::new("cargo");
    command.args(args());

    if is_verbose {
        println!("clippy command: {:?}", command);
    }

    let clippy_output = command.output()?;
    let clippy_string = std::str::from_utf8(&*clippy_output.stdout)?;

    output(clippy_string, &mut std::io::stdout())?;

    Ok(())
}
