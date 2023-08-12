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
/‾‾‾‾  ‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾\
| I'm checking problematic-code v0.1.0 (C:\...\clippy-output\problematic-code)...                               |
| Hmmm... unnecessary trailing semicolon.                                                                       |
|  --> src\main.rs:5:19                                                                                         |
|   |                                                                                                           |
| 5 |     let pi = 3.14;;                                                                                       |
|   |                   ^ You should remove this semicolon                                                      |
|   |                                                                                                           |
|   Note: `#[warn(redundant_semicolons)]` on by default.                                                        |
| Hmmm... unused variable: `pi`.                                                                                |
|  --> src\main.rs:5:9                                                                                          |
|   |                                                                                                           |
| 5 |     let pi = 3.14;;                                                                                       |
|   |         ^^ If this is intentional, prefix it with an underscore: `_pi`                                    |
|   |                                                                                                           |
|   Note: `#[warn(unused_variables)]` on by default.                                                            |
| Hmmm... approximate value of `f{32, 64}::consts::PI` found.                                                   |
|  --> src\main.rs:5:14                                                                                         |
|   |                                                                                                           |
| 5 |     let pi = 3.14;;                                                                                       |
|   |              ^^^^                                                                                         |
|   |                                                                                                           |
|   Hint: consider using the constant directly.                                                                 |
|   Would you like some help with this? Visit                                                                   |
|   https://rust-lang.github.io/rust-clippy/master/index.html#approx_constant.                                  |
|   Note: `#[deny(clippy::approx_constant)]` on by default.                                                     |
| Hmmm... `problematic-code` (bin "problematic-code") generated 2 warnings.                                     |
| Let's fix `problematic-code` (bin "problematic-code")!                                                        |
\_______________________________________________________________________________________________________________/
```

Install with `cargo install clippit`, then in a Rust directory, run

```none
cargo clippit
```

Only tested with `rustc 1.71.1` and `clippy 0.1.71`.

Special thanks to https://github.com/gbigwood/Clippo for the ascii art.