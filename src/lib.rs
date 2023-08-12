#![doc = include_str!("../README.md")]

use crate::clippit_art::ClippyArt;
use regex::{Captures, Regex, Replacer};
use std::borrow::Cow;
use std::io::Write;
use std::mem::swap;
use terminal_size::terminal_size;

pub mod clippit_art;

pub fn output<Writer>(input: &str, output: &mut Writer) -> std::io::Result<()>
    where
        Writer: Write,
{
    let width = u16::min(terminal_size().map(|a| a.0.0).unwrap_or(100), 120);
    let mut clippy = ClippyArt::new(width);

    clippy.add_str(&replace_words(input));
    clippy.finish();
    for s in clippy {
        write!(output, "{s}")?;
    }

    Ok(())
}

/// Replaces words in given string to sound like Clippit.
pub fn replace_words(s: &str) -> String {
    let no_warnings = !s.contains("warning:") && !s.contains("error:");

    let mut result = s.to_string();
    regex_replace_once(&mut result, "^    Checking(.*)", "I'm checking$1...");

    regex_replace_once(
        &mut result,
        r"(?m)^error: aborting due to previous error.*",
        "Sorry, but I cannot continue compiling with that error.",
    );

    regex_replace_once(
        &mut result,
        r"(?m)^error: aborting due to \d* previous errors; \d* warnings emitted",
        "Sorry, but you have too many errors in your code.",
    );

    regex_replace_once(
        &mut result,
        r"(?m)^error: could not compile (.*) due to.*",
        "Let's fix $1!",
    );

    regex_replace_once(
        &mut result,
        "    Finished(.*)\n?$",
        "I finished compiling$1.\n",
    );

    regex_replace(
        &mut result,
        r"(?m)^error: expected (.*), found (.*)",
        "The syntax is wrong because I expected $1 but I found $2.",
    );

    regex_replace(
        &mut result,
        r"(?m)^error\[E0597\]: `(.*)` does not live long enough$",
        "Oops! It looks like the variable with lifetime `$1` is dropped before it is used.",
    );

    regex_replace(
        &mut result,
        r"(?m)^error\[\S+\]: expected (.*), found (.*)",
        "Oops! I expected $1, but I found $2.",
    );

    regex_replace(&mut result, r"(?m)^error\[\S+\]:(.*)", "Oops!$1.");

    regex_replace(&mut result, r"(?m)^help: use (.*)", "Psst... use $1.");

    regex_replace(
        &mut result,
        r"(?m)^(warning|error):(.*)",
        |caps: &Captures| {
            if let Some(s) = caps[2].strip_suffix(" warnings emitted") {
                "You have".to_string() + s + " issues in your code."
            } else if let Some(s) = caps[2].strip_suffix(" warning emitted") {
                "You have".to_string() + s + " issue in your code."
            } else if caps[2].contains('.') {
                "It looks like this could be improved because".to_string() + &caps[2] + "."
            } else {
                "Hmmm...".to_string() + &caps[2] + "."
            }
        },
    );

    regex_replace(&mut result, r"(?m)^(  = )?note:(.*)", |caps: &Captures| {
        let mut result = if caps[0].starts_with("  ") {
            "  Note:"
        } else {
            "Note:"
        }
            .to_string();
        result.push_str(&caps[2]);
        if !caps[2].ends_with('.') && !caps[2].ends_with('?') {
            result.push('.')
        }
        result
    });

    regex_replace(
        &mut result,
        r"(?m)^  = help: for further information visit (.*)",
        "  Would you like some help with this? Visit\n  $1.",
    );

    regex_replace(&mut result, r"(?m)^  = help:(.*)", |caps: &Captures| {
        let mut result = "  Hint:".to_string() + &caps[1];
        if !caps[1].ends_with('?') {
            result.push('.')
        }
        result
    });

    // "^^ help: if this is intentional..."
    regex_replace(&mut result, r"(?m)^  \|( *)(\^+) help: if", "  |$1$2 If");

    // "^ help: remove this semicolon"
    regex_replace(
        &mut result,
        r"(?m)^  \|( *| \|_+)(\^+) help:",
        "  |$1$2 You should",
    );

    regex_replace(
        &mut result,
        r"thread 'main' panicked at ",
        "#$@#$@#$!#$%!@#$ !INTERNAL ERROR! PLEASE REFER TO OWNERS MANUAL\n",
    );

    if no_warnings {
        result.push_str("Woohoo, no warnings!\n");
    }

    result
}

fn regex_replace<R>(str: &mut String, regex: &str, replacement: R)
    where
        R: Replacer,
{
    if let Cow::Owned(mut s) = Regex::new(regex).unwrap().replace_all(str, replacement) {
        swap(str, &mut s);
    }
}

fn regex_replace_once<R>(str: &mut String, regex: &str, replacement: R)
    where
        R: Replacer,
{
    if let Cow::Owned(mut s) = Regex::new(regex).unwrap().replace(str, replacement) {
        swap(str, &mut s);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    // 1
    #[case("", "Woohoo, no warnings!\n")]
    // 2
    #[case(
    r#"    Checking playground v0.0.1 (/playground)
    Finished dev [unoptimized + debuginfo] target(s) in 1.40s
"#,
    // Expected
    r#"I'm checking playground v0.0.1 (/playground)...
I finished compiling dev [unoptimized + debuginfo] target(s) in 1.40s.
Woohoo, no warnings!
"#
    )]
    // 3
    /*
    fn main() {
        .
    }
    */
    #[case(
    r#"    Checking playground v0.0.1 (/playground)
error: expected item, found `.`
 --> src/lib.rs:1:1
  |
1 | .
  | ^ expected item

error: could not compile `playground` (lib) due to previous error
"#,
    // Expected
    r#"I'm checking playground v0.0.1 (/playground)...
The syntax is wrong because I expected item but I found `.`.
 --> src/lib.rs:1:1
  |
1 | .
  | ^ expected item

Let's fix `playground` (lib)!
"#
    )]
    // 4
    /*
    fn main() {
        println!("{}", ((0)));;
    }
     */
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
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#double_parens
  = note: `#[warn(clippy::double_parens)]` on by default

warning: `playground` (bin "playground") generated 2 warnings
    Finished dev [unoptimized + debuginfo] target(s) in 0.41s
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
  Would you like some help with this? Visit
  https://rust-lang.github.io/rust-clippy/master/index.html#double_parens.
  Note: `#[warn(clippy::double_parens)]` on by default.

Hmmm... `playground` (bin "playground") generated 2 warnings.
I finished compiling dev [unoptimized + debuginfo] target(s) in 0.41s.
"#
    )]
    // 5
    /*
    fn main() {
        let mut a = 0;
        let mut b = 0;
        a = b;
        b = a;
        let pi = 3.14;
        if 100 > i32::MAX {}
    }
     */
    #[case(
    r#"    Checking playground v0.0.1 (/playground)
warning: value assigned to `a` is never read
 --> src/main.rs:2:13
  |
2 |     let mut a = 0;
  |             ^
  |
  = help: maybe it is overwritten before being read?
  = note: `#[warn(unused_assignments)]` on by default

warning: value assigned to `b` is never read
 --> src/main.rs:5:5
  |
5 |     b = a;
  |     ^
  |
  = help: maybe it is overwritten before being read?

warning: unused variable: `pi`
 --> src/main.rs:6:9
  |
6 |     let pi = 3.14;
  |         ^^ help: if this is intentional, prefix it with an underscore: `_pi`
  |
  = note: `#[warn(unused_variables)]` on by default

error: this looks like you are trying to swap `a` and `b`
 --> src/main.rs:4:5
  |
4 | /     a = b;
5 | |     b = a;
  | |_________^ help: try: `std::mem::swap(&mut a, &mut b)`
  |
  = note: or maybe you should use `std::mem::replace`?
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#almost_swapped
  = note: `#[deny(clippy::almost_swapped)]` on by default

error: approximate value of `f{32, 64}::consts::PI` found
 --> src/main.rs:6:14
  |
6 |     let pi = 3.14;
  |              ^^^^
  |
  = help: consider using the constant directly
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#approx_constant
  = note: `#[deny(clippy::approx_constant)]` on by default

error: this comparison involving the minimum or maximum element for this type contains a case that is always true or always false
 --> src/main.rs:7:8
  |
7 |     if 100 > i32::MAX {}
  |        ^^^^^^^^^^^^^^
  |
  = help: because `i32::MAX` is the maximum value for this type, this comparison is always false
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#absurd_extreme_comparisons
  = note: `#[deny(clippy::absurd_extreme_comparisons)]` on by default

warning: `playground` (bin "playground") generated 3 warnings
error: could not compile `playground` (bin "playground") due to 3 previous errors; 3 warnings emitted
"#,
    // Expected
    r#"I'm checking playground v0.0.1 (/playground)...
Hmmm... value assigned to `a` is never read.
 --> src/main.rs:2:13
  |
2 |     let mut a = 0;
  |             ^
  |
  Hint: maybe it is overwritten before being read?
  Note: `#[warn(unused_assignments)]` on by default.

Hmmm... value assigned to `b` is never read.
 --> src/main.rs:5:5
  |
5 |     b = a;
  |     ^
  |
  Hint: maybe it is overwritten before being read?

Hmmm... unused variable: `pi`.
 --> src/main.rs:6:9
  |
6 |     let pi = 3.14;
  |         ^^ If this is intentional, prefix it with an underscore: `_pi`
  |
  Note: `#[warn(unused_variables)]` on by default.

Hmmm... this looks like you are trying to swap `a` and `b`.
 --> src/main.rs:4:5
  |
4 | /     a = b;
5 | |     b = a;
  | |_________^ You should try: `std::mem::swap(&mut a, &mut b)`
  |
  Note: or maybe you should use `std::mem::replace`?
  Would you like some help with this? Visit
  https://rust-lang.github.io/rust-clippy/master/index.html#almost_swapped.
  Note: `#[deny(clippy::almost_swapped)]` on by default.

Hmmm... approximate value of `f{32, 64}::consts::PI` found.
 --> src/main.rs:6:14
  |
6 |     let pi = 3.14;
  |              ^^^^
  |
  Hint: consider using the constant directly.
  Would you like some help with this? Visit
  https://rust-lang.github.io/rust-clippy/master/index.html#approx_constant.
  Note: `#[deny(clippy::approx_constant)]` on by default.

Hmmm... this comparison involving the minimum or maximum element for this type contains a case that is always true or always false.
 --> src/main.rs:7:8
  |
7 |     if 100 > i32::MAX {}
  |        ^^^^^^^^^^^^^^
  |
  Hint: because `i32::MAX` is the maximum value for this type, this comparison is always false.
  Would you like some help with this? Visit
  https://rust-lang.github.io/rust-clippy/master/index.html#absurd_extreme_comparisons.
  Note: `#[deny(clippy::absurd_extreme_comparisons)]` on by default.

Hmmm... `playground` (bin "playground") generated 3 warnings.
Let's fix `playground` (bin "playground")!
"#
    )]
    // 6
    /*
    #[deny(clippy::drop_copy)]
    fn main() {
        let x = 1;
        std::mem::drop(x);
    }
     */
    #[case(
    r#"    Checking playground v0.0.1 (/playground)
warning: lint `clippy::drop_copy` has been renamed to `dropping_copy_types`
 --> src/main.rs:1:8
  |
1 | #[deny(clippy::drop_copy)]
  |        ^^^^^^^^^^^^^^^^^ help: use the new name: `dropping_copy_types`
  |
  = note: `#[warn(renamed_and_removed_lints)]` on by default

error: calls to `std::mem::drop` with a value that implements `Copy` does nothing
 --> src/main.rs:4:5
  |
4 |     std::mem::drop(x);
  |     ^^^^^^^^^^^^^^^-^
  |                    |
  |                    argument has type `i32`
  |
  = note: use `let _ = ...` to ignore the expression or result
note: the lint level is defined here
 --> src/main.rs:1:8
  |
1 | #[deny(clippy::drop_copy)]
  |        ^^^^^^^^^^^^^^^^^

warning: `playground` (bin "playground") generated 1 warning
error: could not compile `playground` (bin "playground") due to previous error; 1 warning emitted
"#,
    // Expected
    r#"I'm checking playground v0.0.1 (/playground)...
Hmmm... lint `clippy::drop_copy` has been renamed to `dropping_copy_types`.
 --> src/main.rs:1:8
  |
1 | #[deny(clippy::drop_copy)]
  |        ^^^^^^^^^^^^^^^^^ You should use the new name: `dropping_copy_types`
  |
  Note: `#[warn(renamed_and_removed_lints)]` on by default.

Hmmm... calls to `std::mem::drop` with a value that implements `Copy` does nothing.
 --> src/main.rs:4:5
  |
4 |     std::mem::drop(x);
  |     ^^^^^^^^^^^^^^^-^
  |                    |
  |                    argument has type `i32`
  |
  Note: use `let _ = ...` to ignore the expression or result.
Note: the lint level is defined here.
 --> src/main.rs:1:8
  |
1 | #[deny(clippy::drop_copy)]
  |        ^^^^^^^^^^^^^^^^^

Hmmm... `playground` (bin "playground") generated 1 warning.
Let's fix `playground` (bin "playground")!
"#
    )]
    // 7 Check for false positives in string literal
    /*
    fn main() {
        println!("error: aborting due to previous error{}", 0 as i32);
        println!("= help:{}", match 0 {_ => 0});
    }
     */
    #[case(
    r#"    Checking playground v0.0.1 (/playground)
warning: casting integer literal to `i32` is unnecessary
 --> src/main.rs:2:57
  |
2 |     println!("error: aborting due to previous error{}", 0 as i32);
  |                                                         ^^^^^^^^ help: try: `0_i32`
  |
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_cast
  = note: `#[warn(clippy::unnecessary_cast)]` on by default

warning: this match could be replaced by its body itself
 --> src/main.rs:3:27
  |
3 |     println!("= help:{}", match 0 {_ => 0});
  |                           ^^^^^^^^^^^^^^^^ help: consider using the match body instead: `0`
  |
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#match_single_binding
  = note: `#[warn(clippy::match_single_binding)]` on by default

warning: `playground` (bin "playground") generated 2 warnings (run `cargo clippy --fix --bin "playground"` to apply 2 suggestions)
    Finished dev [unoptimized + debuginfo] target(s) in 0.59s
"#,
    // Expected
    r#"I'm checking playground v0.0.1 (/playground)...
Hmmm... casting integer literal to `i32` is unnecessary.
 --> src/main.rs:2:57
  |
2 |     println!("error: aborting due to previous error{}", 0 as i32);
  |                                                         ^^^^^^^^ You should try: `0_i32`
  |
  Would you like some help with this? Visit
  https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_cast.
  Note: `#[warn(clippy::unnecessary_cast)]` on by default.

Hmmm... this match could be replaced by its body itself.
 --> src/main.rs:3:27
  |
3 |     println!("= help:{}", match 0 {_ => 0});
  |                           ^^^^^^^^^^^^^^^^ You should consider using the match body instead: `0`
  |
  Would you like some help with this? Visit
  https://rust-lang.github.io/rust-clippy/master/index.html#match_single_binding.
  Note: `#[warn(clippy::match_single_binding)]` on by default.

Hmmm... `playground` (bin "playground") generated 2 warnings (run `cargo clippy --fix --bin "playground"` to apply 2 suggestions).
I finished compiling dev [unoptimized + debuginfo] target(s) in 0.59s.
"#
    )]
    // 8
    /*
    fn main() {
        let mut b = &0;
        {
            let a = 0;
            b = &a;
        }
        println!("{}", b);
    }
     */
    #[case(
    r#"    Checking playground v0.0.1 (/playground)
warning: value assigned to `b` is never read
 --> src/main.rs:2:13
  |
2 |     let mut b = &0;
  |             ^
  |
  = help: maybe it is overwritten before being read?
  = note: `#[warn(unused_assignments)]` on by default

error[E0597]: `a` does not live long enough
 --> src/main.rs:5:13
  |
4 |         let a = 0;
  |             - binding `a` declared here
5 |         b = &a;
  |             ^^ borrowed value does not live long enough
6 |     }
  |     - `a` dropped here while still borrowed
7 |     println!("{}", b);
  |                    - borrow later used here

For more information about this error, try `rustc --explain E0597`.
warning: `playground` (bin "playground") generated 1 warning
error: could not compile `playground` (bin "playground") due to previous error; 1 warning emitted
"#,
    // Expected
    r#"I'm checking playground v0.0.1 (/playground)...
Hmmm... value assigned to `b` is never read.
 --> src/main.rs:2:13
  |
2 |     let mut b = &0;
  |             ^
  |
  Hint: maybe it is overwritten before being read?
  Note: `#[warn(unused_assignments)]` on by default.

Oops! It looks like the variable with lifetime `a` is dropped before it is used.
 --> src/main.rs:5:13
  |
4 |         let a = 0;
  |             - binding `a` declared here
5 |         b = &a;
  |             ^^ borrowed value does not live long enough
6 |     }
  |     - `a` dropped here while still borrowed
7 |     println!("{}", b);
  |                    - borrow later used here

For more information about this error, try `rustc --explain E0597`.
Hmmm... `playground` (bin "playground") generated 1 warning.
Let's fix `playground` (bin "playground")!
"#
    )]
    // 10
    /*
    fn main() {
        println();
    }
     */
    #[case(
    r#"    Checking playground v0.0.1 (/playground)
error[E0423]: expected function, found macro `println`
 --> src/main.rs:2:5
  |
2 |     println();
  |     ^^^^^^^ not a function
  |
help: use `!` to invoke the macro
  |
2 |     println!();
  |            +

For more information about this error, try `rustc --explain E0423`.
error: could not compile `playground` (bin "playground") due to previous error
"#,
    r#"I'm checking playground v0.0.1 (/playground)...
Oops! I expected function, but I found macro `println`.
 --> src/main.rs:2:5
  |
2 |     println();
  |     ^^^^^^^ not a function
  |
Psst... use `!` to invoke the macro.
  |
2 |     println!();
  |            +

For more information about this error, try `rustc --explain E0423`.
Let's fix `playground` (bin "playground")!
"#
    )]
    // 11
    // Unable to produce such output on newest clippy but keeping this case
    #[case(
    r#"thread 'main' panicked at 'Usage of `--fix` requires `-Z unstable-options`', src/tools/clippy/src/main.rs:92:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
"#,
    r#"#$@#$@#$!#$%!@#$ !INTERNAL ERROR! PLEASE REFER TO OWNERS MANUAL
'Usage of `--fix` requires `-Z unstable-options`', src/tools/clippy/src/main.rs:92:13
Note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
Woohoo, no warnings!
"#
    )]
    fn test_replace_words(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(replace_words(input), expected);
    }
}
