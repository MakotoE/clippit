use anyhow::Result;
use clippy_output::ClippyOutput;
use std::io::{stdin, Read};
use terminal_size::terminal_size;

/// Outputs ascii art of Clippy saying the input string.
fn main() -> Result<()> {
    let stdin = stdin();
    let mut stdin = stdin.lock();
    let mut line = String::new();

    let width = u16::min(terminal_size().map(|a| a.0 .0).unwrap_or(100), 100);
    let mut clippy = ClippyOutput::new(width);

    loop {
        if stdin.read_to_string(&mut line)? == 0 {
            break;
        }

        clippy.add_str(&line);
        line.clear();

        for s in clippy.by_ref() {
            print!("{}", s);
        }
    }

    clippy.finish();
    for s in clippy {
        print!("{}", s);
    }
    Ok(())
}
