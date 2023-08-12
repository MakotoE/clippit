# clippit 📎

[![Latest version](https://img.shields.io/crates/v/clippit.svg)](https://crates.io/crates/clippit)

`clippit` makes `cargo clippy` sound like Office 2003's Clippit assistant (aka "Clippy").

```none
/‾‾\
|  |
@  @
|| |/
|| ||
|\_/|
\___/
  /\
/‾  ‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾\
| I'm checking rs-test v0.1.0 (/home/makoto/Downloads/rs-test)...                |
| Hmmm... unused variable: `a`.                                                  |
|  --> src/main.rs:2:9                                                           |
|   |                                                                            |
| 2 |     let a = 3.14;                                                          |
|   |         ^ If this is intentional, prefix it with an underscore: `_a`       |
|   |                                                                            |
|   Note: `#[warn(unused_variables)]` on by default.                             |
|                                                                                |
| It looks like this could be improved because approximate value of `f{32, 64}:: |
| consts::PI` found. Consider using it directly.                                 |
|  --> src/main.rs:2:13                                                          |
|   |                                                                            |
| 2 |     let a = 3.14;                                                          |
|   |             ^^^^                                                           |
|   |                                                                            |
|   Note: `#[deny(clippy::approx_constant)]` on by default.                      |
|   Would you like some help with this? Visit                                    |
|   https://rust-lang.github.io/rust-clippy/master/index.html#approx_constant.   |
|                                                                                |
| Sorry, but I cannot continue compiling with that error.                        |
|                                                                                |
| Let's fix `rs-test`!                                                           |
|                                                                                |
| To learn more, run the command again with --verbose.                           |
\________________________________________________________________________________/
```

Install with `cargo install clippit`, then in a Rust directory, run

```
cargo clippy 2>&1 | clippit
```

Only tested with `rustc 1.53.0` and `clippy 0.1.53`.

Special thanks to https://github.com/gbigwood/Clippo for the ascii art.

The current version of clippit only works with a select number of clippy rules, so please create an issue if you come
across a clippy output that could be improved (include the output of `cargo clippy 2>&1`
and `cargo clippy 2>&1 | clippit` in the issue). 