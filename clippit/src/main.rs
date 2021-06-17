use anyhow::Result;
use clippy_output::ClippyOutput;
use std::process::{Command, ExitStatus, Stdio};
use terminal_size::terminal_size;

fn main() -> Result<()> {
    std::process::exit(if run()?.success() { 0 } else { 1 })
}

/// Returns exit status of child process
fn run() -> Result<ExitStatus> {
    let output = Command::new("cargo")
        .args(&["clippy"])
        .stderr(Stdio::piped())
        .output()?;

    let width = u16::min(terminal_size().map(|a| a.0 .0).unwrap_or(100), 120);
    let mut clippy = ClippyOutput::new(width);

    let str = replace_words(std::str::from_utf8(&output.stderr)?);
    clippy.add_str(&str);
    clippy.finish();
    for s in clippy {
        print!("{}", s);
    }

    Ok(output.status)
}

fn replace_words(s: &str) -> String {
    // Replace "Checking"
    let mut result = if let Some(n) = s.find(|c: char| c == '\n' || c == '\r') {
        s[..n].replacen("    Checking", "I'm checking", 1) + "...\n" + &s[n..]
    } else {
        s.to_string()
    };

    if result.contains("could not compile") {
        // Compilation error

        result.replace(
            "error: aborting due to previous error",
            "Sorry, but I can't compile with that error!",
        );

        result = result.replacen(
            "error: expected",
            "Hmmm... The syntax is wrong because I expected",
            error_count - 1,
        );
    } else {
        // The cargo clippy output can contain either:
        // 2 or more "warning:"
        // 2 or more "error:"
        // none of the above
        //
        // If the string contains "warning:" or "error:", the last match should not be changed

        let error_count = result.matches("error:").count();
        if error_count > 0 {
            result = result.replacen("error:", "Hmmm... ", error_count - 1);
        } else {
            let warning_count = s.matches("warning:").count();
            if warning_count > 0 {
                result = result.replacen(
                    "warning:",
                    "It looks like this could be improved because",
                    warning_count - 1,
                );
            }
        }
    }

    result
}
