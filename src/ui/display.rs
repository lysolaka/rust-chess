use crate::core::board::Board;

use super::input;
use super::error::QuitGame;

pub fn game_loop(board: &mut Board) {
    loop {
        board.print();
        println!();
        println!("Current move is: {}", board.current_move());

        match input::query_input() {
            Ok(p) => {
                if let Err(e) = board.move_piece(p.0, p.1) {
                    println!("Movement failed, reason: {e}");
                }
            }
            Err(e) => {
                println!("Parsing position failed, reason: {e}");
                if e.is::<QuitGame>() {
                    break;
                } else {
                }
            }
        }
        println!();
    }
}
