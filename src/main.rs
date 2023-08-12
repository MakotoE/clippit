#![warn(clippy::pedantic)]

use std::env::args;
use std::io::Write;
use anyhow::Result;
use clippit::{output};
use std::process::{Command};

/// Use -v to see the `cargo clippy` command and output.
fn main() -> Result<()> {
    let args: Vec<String> = args().skip(1).collect();
    std::process::exit(run(args, &mut std::io::stderr())?);
}

fn run<Writer>(mut args: Vec<String>, writer: &mut Writer) -> Result<i32>
    where Writer: Write,
{
    args.insert(0, "clippy".to_string());

    let is_verbose = args.iter().any(|arg| { arg == "-v" || arg == "--verbose" });

    let mut command = Command::new("cargo");

    command.args(args);

    let clippy_output = command.output()?;
    let clippy_string = std::str::from_utf8(&*clippy_output.stderr)?;

    if is_verbose {
        eprintln!("clippy command: {:?}", command);
        eprintln!("clippy output: {clippy_string}");
    }

    output(clippy_string, writer)?;
    Ok(clippy_output.status.code().unwrap_or(0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problematic_code() {
        std::env::set_current_dir("problematic-code").unwrap();

        let mut output: Vec<u8> = Vec::new();
        let status_code = run(vec!["-v".to_string()], &mut output).unwrap();

        let output_str = std::str::from_utf8(&output).unwrap();
        println!("{output_str}");

        assert_ne!(status_code, 0);
        assert!(output_str.contains("problematic-code"));
    }
}