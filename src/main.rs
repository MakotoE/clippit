use anyhow::Result;
use std::io::{stdin, Read};

fn main() -> Result<()> {
    let stdin = stdin();
    let mut stdin = stdin.lock();
    let mut line = String::new();

    loop {
        if stdin.read_to_string(&mut line)? == 0 {
            return Ok(());
        }

        print!("{}", line);

        line.clear();
    }
}
