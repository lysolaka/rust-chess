#![allow(dead_code)]

#[cfg(all(feature = "unicode", feature = "ascii"))]
compile_error!("feature \"unicode\" and feature \"ascii\" cannot be enabled at the same time");

use rust_chess::core::board::Board;

fn main() {
    let b = Board::new();
    b.print();
    print!("\n");
}
