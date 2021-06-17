use anyhow::{Error, Result};
use clippy_output::ClippyOutput;
use std::io::Read;
use std::process::{Command, Stdio};
use terminal_size::terminal_size;

fn main() -> Result<()> {
    let mut child = Command::new("cargo")
        .args(&["clippy"])
        .stderr(Stdio::piped())
        .spawn()?;
    let mut stderr = child
        .stderr
        .take()
        .ok_or_else(|| Error::msg("could not get stderr"))?;
    let mut line = String::new();

    let width = u16::min(terminal_size().map(|a| a.0 .0).unwrap_or(100), 100);
    let mut clippy = ClippyOutput::new(width);

    loop {
        if stderr.read_to_string(&mut line)? == 0 {
            break;
        }

        clippy.add_str(&line);
        line.clear();

        for s in clippy.by_ref() {
            print!("{}", s);
        }
    }

    let output = child.wait_with_output()?;
    clippy.add_str(std::str::from_utf8(&output.stderr)?);

    clippy.finish();
    for s in clippy {
        print!("{}", s);
    }

    if output.status.success() {
        Ok(())
    } else {
        Err(Error::msg(""))
    }
}
