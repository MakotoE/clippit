use anyhow::Result;
use clippit::replace_words;
use clippy_output::ClippyOutput;
use std::io::{stdin, Read};
use terminal_size::terminal_size;

fn main() -> Result<()> {
    let mut line = String::new();
    stdin().read_to_string(&mut line)?;

    let width = u16::min(terminal_size().map(|a| a.0 .0).unwrap_or(100), 120);
    let mut clippy = ClippyOutput::new(width);

    clippy.add_str(&replace_words(&line));
    clippy.finish();
    for s in clippy {
        print!("{}", s);
    }
    Ok(())
}
