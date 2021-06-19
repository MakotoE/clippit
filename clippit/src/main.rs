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
    let mut result = if let Some(after_checking) = s.trim_start().strip_prefix("    Checking") {
        let newline_index = after_checking
            .find("\n")
            .unwrap_or(after_checking.len() - 1);
        "I'm checking".to_string()
            + &after_checking[..newline_index]
            + "..."
            + &after_checking[newline_index..]
    } else {
        s.trim_start().to_string()
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

        if let Cow::Owned(s) = Regex::new(r"error\[\S+]:")
            .unwrap()
            .replace_all(&result, "Oops!")
        {
            result = s;
        }
    } else {
        // cargo clippy output
        if let Cow::Owned(s) =
            Regex::new("(warning|error):(.*)")
                .unwrap()
                .replace_all(&result, |caps: &Captures| {
                    if let Some(s) = caps[2].strip_suffix(" warnings emitted") {
                        "You have".to_string() + s + " issues in your code."
                    } else {
                        "It looks like this could be improved because".to_string() + &caps[2] + "."
                    }
                })
        {
            result = s;
        }

        if let Cow::Owned(s) = Regex::new("= note: (.*)")
            .unwrap()
            .replace_all(&result, "Note: $1.")
        {
            result = s;
        }

        if let Cow::Owned(s) = Regex::new("= help: for further information visit (.*)")
            .unwrap()
            .replace_all(&result, "Would you like help with this? Visit\n  $1.")
        {
            result = s;
        }
    }

    // "Finished..."
    let last_line_index = result[..result.len() - 1]
        .rfind("\n")
        .unwrap_or(result.len() - 1)
        + 1;
    const FINISHED: &str = "    Finished";
    if result[last_line_index..].starts_with(FINISHED) {
        result.replace_range(
            last_line_index..last_line_index + FINISHED.len(),
            "I finished compiling",
        );

        if result.ends_with("\n") {
            result.insert(result.len() - 1, '.');
        }
    }

    result
}
