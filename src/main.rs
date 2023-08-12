use anyhow::Result;
use clippit::{output};
use std::io::{stdin, Read};

fn main() -> Result<()> {
    let mut line = String::new();
    stdin().read_to_string(&mut line)?;
    output(&line, &mut std::io::stdout())?;
    Ok(())
}
