# clippit 📎

[![Latest version](https://img.shields.io/crates/v/clippit.svg)](https://crates.io/crates/clippit) [![Documentation](https://docs.rs/clippit/badge.svg)](https://docs.rs/clippit/)

`clippit` makes `cargo clippy` sound like Office 2003's Clippit assistant (aka "Clippy").

```none
$ cargo clippit
   /‾‾\
   |  |
   @  @
   || |/
   || ||
   |\_/|
   \___/
     /\
/‾‾‾‾  ‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾\
| I'm checking problematic-code v0.1.0 (C:\...\clippy-output\problematic-code)...                    |
| Hmmm... unnecessary trailing semicolon.                                                            |
|  --> src\main.rs:5:19                                                                              |
|   |                                                                                                |
| 5 |     let pi = 3.14;;                                                                            |
|   |                   ^ You should remove this semicolon                                           |
|   Hint: because `i32::MAX` is the maximum value for this type, this comparison is always false.    |
|   Would you like some help with this? Visit                                                        |
|   https://rust-lang.github.io/rust-clippy/master/index.html#absurd_extreme_comparisons.            |
|   Note: `#[deny(clippy::absurd_extreme_comparisons)]` on by default.                               |
| Hmmm... `problematic-code` (bin "problematic-code") generated 2 warnings.                          |
| Let's fix `problematic-code` (bin "problematic-code")!                                             |
\____________________________________________________________________________________________________/
```

Install with `cargo install clippit`, then in a Rust directory, run

```none
cargo clippit
```

Only tested with `rustc 1.71.1` and `clippy 0.1.71`.

Special thanks to https://github.com/gbigwood/Clippo for the ascii art.