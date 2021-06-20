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

    if result.contains("https://rust-lang.github.io/rust-clippy/") && result.contains("clippy::") {
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
                        "Hmmm...".to_string() + &caps[2] + "."
                    }
                })
        {
            result = s;
        }

        if let Cow::Owned(s) =
            Regex::new("= note:(.*)")
                .unwrap()
                .replace_all(&result, |caps: &Captures| {
                    let mut result = "Note:".to_string() + &caps[1];
                    if !caps[1].ends_with("?") {
                        result.push('.')
                    }
                    result
                })
        {
            result = s;
        }

        if let Cow::Owned(s) = Regex::new("= help: for further information visit (.*)")
            .unwrap()
            .replace_all(&result, "Would you like help with this? Visit\n  $1.")
        {
            result = s;
        }

        if let Cow::Owned(s) =
            Regex::new("= help:(.*)")
                .unwrap()
                .replace_all(&result, |caps: &Captures| {
                    let mut result = "Hint:".to_string() + &caps[1];
                    if !caps[1].ends_with("?") {
                        result.push('.')
                    }
                    result
                })
        {
            result = s;
        }

        result = result.replace("^ help: if", "^ If");

        result = result.replace("^ help:", "^ You should");
    } else {
        // Compilation error or no clippy output
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
    }

    // "Finished..."
    let last_line_index = match result.strip_suffix("\n").unwrap_or(&result).rfind("\n") {
        Some(n) => n + 1,
        None => result.len(),
    };

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
        // Expected
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
        // Expected
        r#"I'm checking playground v0.0.1 (/playground)...
Hmmm... unnecessary trailing semicolon.
 --> src/main.rs:2:27
  |
2 |     println!("{}", ((0)));;
  |                           ^ You should remove this semicolon
  |
  Note: `#[warn(redundant_semicolons)]` on by default.

Hmmm... consider removing unnecessary double parentheses.
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
    #[case(
        r#"    Checking rs-test v0.1.0 (/home/makoto/Downloads/rs-test)
warning: value assigned to `a` is never read
 --> src/main.rs:3:13
  |
3 |     let mut a = 0;
  |             ^
  |
  = note: `#[warn(unused_assignments)]` on by default
  = help: maybe it is overwritten before being read?

warning: value assigned to `b` is never read
 --> src/main.rs:6:5
  |
6 |     b = a;
  |     ^
  |
  = help: maybe it is overwritten before being read?

warning: unused variable: `pi`
 --> src/main.rs:7:9
  |
7 |     let pi = 3.14;
  |         ^^ help: if this is intentional, prefix it with an underscore: `_pi`
  |
  = note: `#[warn(unused_variables)]` on by default

error: this looks like you are trying to swap `a` and `b`
 --> src/main.rs:5:5
  |
5 | /     a = b;
6 | |     b = a;
  | |_________^ help: try: `std::mem::swap(&mut a, &mut b)`
  |
  = note: `#[deny(clippy::almost_swapped)]` on by default
  = note: or maybe you should use `std::mem::replace`?
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#almost_swapped

error: this comparison involving the minimum or maximum element for this type contains a case that is always true or always false
 --> src/main.rs:2:8
  |
2 |     if 100 > i32::MAX {}
  |        ^^^^^^^^^^^^^^
  |
  = note: `#[deny(clippy::absurd_extreme_comparisons)]` on by default
  = help: because `i32::MAX` is the maximum value for this type, this comparison is always false
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#absurd_extreme_comparisons

error: approximate value of `f{32, 64}::consts::PI` found. Consider using it directly
 --> src/main.rs:7:14
  |
7 |     let pi = 3.14;
  |              ^^^^
  |
  = note: `#[deny(clippy::approx_constant)]` on by default
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#approx_constant

error: aborting due to 3 previous errors; 3 warnings emitted

error: could not compile `rs-test`

To learn more, run the command again with --verbose.
"#,
    // Expected
        r#"I'm checking rs-test v0.1.0 (/home/makoto/Downloads/rs-test)...
Hmmm... value assigned to `a` is never read.
 --> src/main.rs:3:13
  |
3 |     let mut a = 0;
  |             ^
  |
  Note: `#[warn(unused_assignments)]` on by default.
  Hint: maybe it is overwritten before being read?

Hmmm... value assigned to `b` is never read.
 --> src/main.rs:6:5
  |
6 |     b = a;
  |     ^
  |
  Hint: maybe it is overwritten before being read?

Hmmm... unused variable: `pi`.
 --> src/main.rs:7:9
  |
7 |     let pi = 3.14;
  |         ^^ If this is intentional, prefix it with an underscore: `_pi`
  |
  Note: `#[warn(unused_variables)]` on by default.

Hmmm... this looks like you are trying to swap `a` and `b`.
 --> src/main.rs:5:5
  |
5 | /     a = b;
6 | |     b = a;
  | |_________^ You should try: `std::mem::swap(&mut a, &mut b)`
  |
  Note: `#[deny(clippy::almost_swapped)]` on by default.
  Note: or maybe you should use `std::mem::replace`?
  Would you like help with this? Visit
  https://rust-lang.github.io/rust-clippy/master/index.html#almost_swapped.

Hmmm... this comparison involving the minimum or maximum element for this type contains a case that is always true or always false.
 --> src/main.rs:2:8
  |
2 |     if 100 > i32::MAX {}
  |        ^^^^^^^^^^^^^^
  |
  Note: `#[deny(clippy::absurd_extreme_comparisons)]` on by default.
  Hint: because `i32::MAX` is the maximum value for this type, this comparison is always false.
  Would you like help with this? Visit
  https://rust-lang.github.io/rust-clippy/master/index.html#absurd_extreme_comparisons.

It looks like this could be improved because approximate value of `f{32, 64}::consts::PI` found. Consider using it directly.
 --> src/main.rs:7:14
  |
7 |     let pi = 3.14;
  |              ^^^^
  |
  Note: `#[deny(clippy::approx_constant)]` on by default.
  Would you like help with this? Visit
  https://rust-lang.github.io/rust-clippy/master/index.html#approx_constant.

You have aborting due to 3 previous errors; 3 issues in your code.

Hmmm... could not compile `rs-test`.

To learn more, run the command again with --verbose.
"#
    )]
    fn test_replace_words(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(replace_words(input), expected);
    }
}
