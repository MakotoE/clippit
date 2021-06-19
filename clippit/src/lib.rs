use regex::{Captures, Regex};
use std::borrow::Cow;

pub fn replace_words(s: &str) -> String {
    // Replace "Checking"
    let mut result = if let Some(after_checking) = s.strip_prefix("    Checking") {
        let newline_index = after_checking.find("\n").unwrap_or(after_checking.len());
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

        if let Cow::Owned(s) = Regex::new("error: expected (.*)")
            .unwrap()
            .replace_all(&result, "The syntax is wrong because I expected $1.")
        {
            result = s;
        }

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
                    } else if caps[2].contains(".") {
                        "It looks like this could be improved because".to_string() + &caps[2] + "."
                    } else {
                        "Hmmm...".to_string() + &caps[2]
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

        result = result.replace("^ help: ", "^ You should ");
    }

    // "Finished..."
    let last_line_index = match result.strip_suffix("\n").unwrap_or(&result).rfind("\n") {
        Some(n) => n + 1,
        None => result.len(),
    };

    // .unwrap_or(result.len() - 2)
    // + 1;
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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("", "")]
    #[case(
        r#"    Checking playground v0.0.1 (/playground)
    Finished dev [unoptimized + debuginfo] target(s) in 1.40s
"#,
        r#"I'm checking playground v0.0.1 (/playground)...
I finished compiling dev [unoptimized + debuginfo] target(s) in 1.40s.
"#
    )]
    #[case(
        r#"    Checking playground v0.0.1 (/playground)
error: expected expression, found `.`
 --> src/main.rs:2:5
  |
2 |     .
  |     ^ expected expression

error: aborting due to previous error

error: could not compile `playground`

To learn more, run the command again with --verbose.
"#,
        r#"I'm checking playground v0.0.1 (/playground)...
The syntax is wrong because I expected expression, found `.`.
 --> src/main.rs:2:5
  |
2 |     .
  |     ^ expected expression

Sorry, but I cannot continue compiling with that error.

Let's fix `playground`!

To learn more, run the command again with --verbose.
"#
    )]
    #[case(
    r#"    Checking playground v0.0.1 (/playground)
warning: unnecessary trailing semicolon
 --> src/main.rs:2:27
  |
2 |     println!("{}", ((0)));;
  |                           ^ help: remove this semicolon
  |
  = note: `#[warn(redundant_semicolons)]` on by default

warning: consider removing unnecessary double parentheses
 --> src/main.rs:2:20
  |
2 |     println!("{}", ((0)));;
  |                    ^^^^^
  |
  = note: `#[warn(clippy::double_parens)]` on by default
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#double_parens

warning: 2 warnings emitted

    Finished dev [unoptimized + debuginfo] target(s) in 0.54s
"#,
    r#"I'm checking playground v0.0.1 (/playground)...
Hmmm... unnecessary trailing semicolon
 --> src/main.rs:2:27
  |
2 |     println!("{}", ((0)));;
  |                           ^ You should remove this semicolon
  |
  Note: `#[warn(redundant_semicolons)]` on by default.

Hmmm... consider removing unnecessary double parentheses
 --> src/main.rs:2:20
  |
2 |     println!("{}", ((0)));;
  |                    ^^^^^
  |
  Note: `#[warn(clippy::double_parens)]` on by default.
  Would you like help with this? Visit
  https://rust-lang.github.io/rust-clippy/master/index.html#double_parens.

You have 2 issues in your code.

I finished compiling dev [unoptimized + debuginfo] target(s) in 0.54s.
"#
    )]
    fn test_replace_words(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(replace_words(input), expected);
    }
}
