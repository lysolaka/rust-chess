#[cfg(all(feature = "unicode", feature = "ascii"))]
compile_error!("feature \"unicode\" and feature \"ascii\" cannot be enabled at the same time");

use rust_chess::core::board::Board;
use rust_chess::ui::display;

fn main() {
    let mut b = Box::new(Board::new());
    display::game_loop(&mut b);
}
