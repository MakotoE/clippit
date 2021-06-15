use anyhow::{Error, Result};
use clippy_output::ClippyOutput;
use std::io::{stdin, Read};
use terminal_size::terminal_size;

fn main() -> Result<()> {
    let stdin = stdin();
    let mut stdin = stdin.lock();
    let mut line = String::new();

    let (width, _) = terminal_size().ok_or_else(|| Error::msg("not a tty"))?;
    let mut clippy = ClippyOutput::new(u16::min(width.0, 100));

    loop {
        if stdin.read_to_string(&mut line)? == 0 {
            break;
        }

        clippy.add_str(&line);

        for s in clippy.by_ref() {
            print!("{}", s);
        }

        line.clear();
    }

    clippy.finish();
    for s in clippy {
        print!("{}", s);
    }
    Ok(())
}
