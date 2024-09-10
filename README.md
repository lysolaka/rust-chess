# RUST-CHESS
A very crude chess game to show how Rust works compared to C++

See also: [lysolaka/cpp-chess](https://github.com/lysolaka/cpp-chess)

## Building

Build and run using `cargo run`. Sorry I don't know how to package Rust yet.

If your terminal doesn't support unicode, edit `Cargo.toml` and to look like this:
```
...

[features]
default = ["ascii"]
unicode = []
ascii = []
```
