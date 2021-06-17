use anyhow::Result;
use clippy_output::ClippyOutput;
use regex::{Captures, Regex};
use std::borrow::Cow;
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

fn replace_words(s: &str) -> String {
    // Replace "Checking"
    let trimmed = s.trim_start();
    let mut result = if let Some(after_checking) = trimmed.strip_prefix("Checking") {
        let newline_index = after_checking
            .find("\n")
            .unwrap_or(after_checking.len() - 1);
        "I'm checking".to_string()
            + &after_checking[..newline_index]
            + "..."
            + &after_checking[newline_index..]
    } else {
        trimmed.to_string()
    };

    if result.contains("could not compile") {
        // Compilation error

        result = result.replace(
            "error: aborting due to previous error",
            "Sorry, but I cannot continue compiling with that error.",
        );

        if let Cow::Owned(s) = Regex::new("error: could not compile (.*)")
            .unwrap()
            .replace_all(&result, "Let's fix $1!")
        {
            result = s;
        }

        result = result.replace("error: expected", "The syntax is wrong because I expected");

        if let Cow::Owned(s) = Regex::new(r#"error\[\S+]:"#)
            .unwrap()
            .replace_all(&result, "Oops!")
        {
            result = s;
        }
    } else {
        if let Cow::Owned(s) =
            Regex::new("(warning|error):(.*)")
                .unwrap()
                .replace_all(&result, |caps: &Captures| {
                    if caps[0].ends_with("warnings emitted") || caps[0].ends_with("errors emitted")
                    {
                        (&caps[0]).to_string()
                    } else {
                        "It looks like this could be improved because".to_string() + &caps[2] + "."
                    }
                })
        {
            result = s;
        }
    }

    result
}
