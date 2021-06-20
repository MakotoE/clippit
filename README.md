`clippit` makes `cargo clippy` sound like Office 2003's Clippit assistant (aka "Clippy").

```
/‾‾\
|  |
@  @
|| |/
|| ||
|\_/|
\___/
  /\
/‾  ‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾\
| Hmmm... unused variable: `a`.                                                                            |
|  --> src/main.rs:2:9                                                                                     |
|   |                                                                                                      |
| 2 |     let a = 0;                                                                                       |
|   |         ^ If this is intentional, prefix it with an underscore: `_a`                                 |
|   |                                                                                                      |
|   Note: `#[warn(unused_variables)]` on by default.                                                       |
|                                                                                                          |
| You have 1 issue in your code.                                                                           |
|                                                                                                          |
| I finished compiling dev [unoptimized + debuginfo] target(s) in 0.00s.                                   |
\__________________________________________________________________________________________________________/
```

Install with `cargo install clippit`, then in a Rust directory, run

```
cargo clippy 2>&1 | clippit
```

The `clippy-output` directory contains a library and command for adding just the Clippy ascii art to text.

Only tested with `rustc 1.53.0` and `clippy 0.1.53`.

The current version of clippit only works with a select number of clippy rules, so please create an issue if you come across a clippy output that could be improved (include the output of `cargo clippy 2>&1` and `cargo clippy 2>&1 | clippit` in the issue). 