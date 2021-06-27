use regex::{Captures, Regex, Replacer};
use std::borrow::Cow;
use std::mem::swap;

/// Replaces words in given string to sound like Clippy.
pub fn replace_words(s: &str) -> String {
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
        r"(?m)^error: could not compile (.*)",
        "Let's fix $1!",
    );

    regex_replace_once(
        &mut result,
        "    Finished(.*)\n?$",
        "I finished compiling$1.\n",
    );

    regex_replace(
        &mut result,
        r"(?m)^error: expected (.*)",
        "The syntax is wrong because I expected $1.",
    );

    regex_replace(&mut result, r"(?m)^error\[\S+]:(.*)", "Oops!$1.");

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
        if !caps[2].ends_with('?') {
            result.push('.')
        }
        result
    });

    regex_replace(
        &mut result,
        r"(?m)^  = help: for further information visit (.*)",
        "  Would you like some help with this? Visit\n$1.",
    );

    regex_replace(&mut result, r"(?m)^  = help:(.*)", |caps: &Captures| {
        let mut result = "  Hint:".to_string() + &caps[1];
        if !caps[1].ends_with('?') {
            result.push('.')
        }
        result
    });

    regex_replace(&mut result, r"(?m)^  \|( *)(\^+) help: if", "  |$1$2 If");
    regex_replace(
        &mut result,
        r"(?m)^  \|( *| \|_+)(\^+) help:",
        "  |$1$2 You should",
    );
    result
}

fn regex_replace<R>(str: &mut String, regex: &str, replacement: R)
where
    R: Replacer,
{
    if let Cow::Owned(mut s) = Regex::new(regex).unwrap().replace_all(&str, replacement) {
        swap(str, &mut s);
    }
}

fn regex_replace_once<R>(str: &mut String, regex: &str, replacement: R)
where
    R: Replacer,
{
    if let Cow::Owned(mut s) = Regex::new(regex).unwrap().replace(&str, replacement) {
        swap(str, &mut s);
    }
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

error: aborting due to previous error; 1 warning emitted

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
  Would you like some help with this? Visit
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
  Would you like some help with this? Visit
  https://rust-lang.github.io/rust-clippy/master/index.html#almost_swapped.

Hmmm... this comparison involving the minimum or maximum element for this type contains a case that is always true or always false.
 --> src/main.rs:2:8
  |
2 |     if 100 > i32::MAX {}
  |        ^^^^^^^^^^^^^^
  |
  Note: `#[deny(clippy::absurd_extreme_comparisons)]` on by default.
  Hint: because `i32::MAX` is the maximum value for this type, this comparison is always false.
  Would you like some help with this? Visit
  https://rust-lang.github.io/rust-clippy/master/index.html#absurd_extreme_comparisons.

It looks like this could be improved because approximate value of `f{32, 64}::consts::PI` found. Consider using it directly.
 --> src/main.rs:7:14
  |
7 |     let pi = 3.14;
  |              ^^^^
  |
  Note: `#[deny(clippy::approx_constant)]` on by default.
  Would you like some help with this? Visit
  https://rust-lang.github.io/rust-clippy/master/index.html#approx_constant.

Sorry, but you have too many errors in your code.

Let's fix `rs-test`!

To learn more, run the command again with --verbose.
"#
    )]
    #[case(
        r#"    Checking rs-test v0.1.0 (/home/makoto/Downloads/rs-test)
warning: unused variable: `a`
 --> src/main.rs:2:9
  |
2 |     let a = 0;
  |         ^ help: if this is intentional, prefix it with an underscore: `_a`
  |
  = note: `#[warn(unused_variables)]` on by default

warning: 1 warning emitted

    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
"#,
        // Expected
        r#"I'm checking rs-test v0.1.0 (/home/makoto/Downloads/rs-test)...
Hmmm... unused variable: `a`.
 --> src/main.rs:2:9
  |
2 |     let a = 0;
  |         ^ If this is intentional, prefix it with an underscore: `_a`
  |
  Note: `#[warn(unused_variables)]` on by default.

You have 1 issue in your code.

I finished compiling dev [unoptimized + debuginfo] target(s) in 0.02s.
"#
    )]
    #[case(
        r#"    Checking rs-test v0.1.0 (/home/makoto/Downloads/rs-test)
error: calls to `std::mem::drop` with a value that implements `Copy`. Dropping a copy leaves the original intact
 --> src/main.rs:3:5
  |
3 |     std::mem::drop(x);
  |     ^^^^^^^^^^^^^^^^^
  |
  = note: `#[deny(clippy::drop_copy)]` on by default
note: argument has type i32
 --> src/main.rs:3:20
  |
3 |     std::mem::drop(x);
  |                    ^
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#drop_copy

error: aborting due to previous error

error: could not compile `rs-test`

To learn more, run the command again with --verbose.
"#,
        // Expected
        r#"I'm checking rs-test v0.1.0 (/home/makoto/Downloads/rs-test)...
It looks like this could be improved because calls to `std::mem::drop` with a value that implements `Copy`. Dropping a copy leaves the original intact.
 --> src/main.rs:3:5
  |
3 |     std::mem::drop(x);
  |     ^^^^^^^^^^^^^^^^^
  |
  Note: `#[deny(clippy::drop_copy)]` on by default.
Note: argument has type i32.
 --> src/main.rs:3:20
  |
3 |     std::mem::drop(x);
  |                    ^
  Would you like some help with this? Visit
  https://rust-lang.github.io/rust-clippy/master/index.html#drop_copy.

Sorry, but I cannot continue compiling with that error.

Let's fix `rs-test`!

To learn more, run the command again with --verbose.
"#
    )]
    // Check for false positives
    #[case(
        r#"warning: casting integer literal to `i32` is unnecessary
 --> src/main.rs:2:57
  |
2 |     println!("error: aborting due to previous error{}", 0 as i32);
  |                                                         ^^^^^^^^ help: try: `0_i32`
  |
  = note: `#[warn(clippy::unnecessary_cast)]` on by default
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_cast

warning: this match could be replaced by its body itself
 --> src/main.rs:3:27
  |
3 |     println!("= help:{}", match 0 {_ => 0});
  |                           ^^^^^^^^^^^^^^^^ help: consider using the match body instead: `0`
  |
  = note: `#[warn(clippy::match_single_binding)]` on by default
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#match_single_binding

warning: 2 warnings emitted

    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
"#,
        // Expected
        r#"Hmmm... casting integer literal to `i32` is unnecessary.
 --> src/main.rs:2:57
  |
2 |     println!("error: aborting due to previous error{}", 0 as i32);
  |                                                         ^^^^^^^^ You should try: `0_i32`
  |
  Note: `#[warn(clippy::unnecessary_cast)]` on by default.
  Would you like some help with this? Visit
  https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_cast.

Hmmm... this match could be replaced by its body itself.
 --> src/main.rs:3:27
  |
3 |     println!("= help:{}", match 0 {_ => 0});
  |                           ^^^^^^^^^^^^^^^^ You should consider using the match body instead: `0`
  |
  Note: `#[warn(clippy::match_single_binding)]` on by default.
  Would you like some help with this? Visit
  https://rust-lang.github.io/rust-clippy/master/index.html#match_single_binding.

You have 2 issues in your code.

I finished compiling dev [unoptimized + debuginfo] target(s) in 0.00s.
"#
    )]
    // 9
    #[case(
        r#"error[E0597]: `a` does not live long enough
 --> src/main.rs:6:22
  |
6 |         b = function(&a);
  |                      ^^ borrowed value does not live long enough
7 |     }
  |     - `a` dropped here while still borrowed
8 |     println!("{}", b);
  |                    - borrow later used here
"#,
        r#"Oops! `a` does not live long enough.
 --> src/main.rs:6:22
  |
6 |         b = function(&a);
  |                      ^^ borrowed value does not live long enough
7 |     }
  |     - `a` dropped here while still borrowed
8 |     println!("{}", b);
  |                    - borrow later used here
"#
    )]
    fn test_replace_words(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(replace_words(input), expected);
    }
}
