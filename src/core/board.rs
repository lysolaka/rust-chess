use std::usize;

use super::piece;
use super::piece::Piece;
use super::position::Pos;

/// Wraps a chess board implemented as an array of size 64.
/// Each field is an option:
/// - Some means the field has a piece
/// - None means the field is empty
pub struct Board {
    fields: Box<[Option<Piece>; 64]>,
    current_move: piece::Side,
}

impl Board {
    /// Construct a `Board` instance and puts all the pieces in the starting positions.
    pub fn new() -> Self {
        let mut fields = Box::new([None; 64]);

        // Fill in pawns
        let mut fill_pawns = |row: u8| {
            let side = if row == 2 {
                piece::Side::White
            } else if row == 7 {
                piece::Side::Black
            } else {
                panic!("Shouldn't happen");
            };

            for column in 'a'..='h' {
                fields[Pos::at(column, row)] = Some(Piece::new(piece::Type::Pawn, side));
            }
        };

        fill_pawns(2);
        fill_pawns(7);

        // Fill in the rest
        let mut fill_figures = |row: u8| {
            let side = if row == 1 {
                piece::Side::White
            } else if row == 8 {
                piece::Side::Black
            } else {
                panic!("Shouldn't happen");
            };

            fields[Pos::at('a', row)] = Some(Piece::new(piece::Type::Rook, side));
            fields[Pos::at('b', row)] = Some(Piece::new(piece::Type::Knight, side));
            fields[Pos::at('c', row)] = Some(Piece::new(piece::Type::Bishop, side));
            fields[Pos::at('d', row)] = Some(Piece::new(piece::Type::Queen, side));
            fields[Pos::at('e', row)] = Some(Piece::new(piece::Type::King, side));
            fields[Pos::at('f', row)] = Some(Piece::new(piece::Type::Bishop, side));
            fields[Pos::at('g', row)] = Some(Piece::new(piece::Type::Knight, side));
            fields[Pos::at('h', row)] = Some(Piece::new(piece::Type::Rook, side));
        };

        fill_figures(1);
        fill_figures(8);

        Self {
            fields,
            current_move: piece::Side::White,
        }
    }

    /// Returns an optional at a specified position (`pos`).
    /// There is `Some(&Piece)` or `None` (specified position is empty)
    pub fn at(&self, pos: Pos) -> Option<&Piece> {
        self.fields[usize::from(pos)].as_ref()
    }

    /// Returns an optional at a specified position (`pos`).
    /// There is `Some(&mut Piece)` or `None` (specified position is empty).
    /// This method is private and can be used instead of direct indexing
    /// of the `fields` array.
    fn at_mut(&mut self, pos: Pos) -> Option<&mut Piece> {
        self.fields[usize::from(pos)].as_mut()
    }

    /// Prints the state of the board in a visually pleasing format
    ///
    /// # Example - board at its starting position
    /// ```text
    /// +---+---+---+---+---+---+---+---+
    /// | ♜ | ♞ | ♝ | ♛ | ♚ | ♝ | ♞ | ♜ | 8
    /// +---+---+---+---+---+---+---+---+
    /// | ♟ | ♟ | ♟ | ♟ | ♟ | ♟ | ♟ | ♟ | 7
    /// +---+---+---+---+---+---+---+---+
    /// |   |   |   |   |   |   |   |   | 6
    /// +---+---+---+---+---+---+---+---+
    /// |   |   |   |   |   |   |   |   | 5
    /// +---+---+---+---+---+---+---+---+
    /// |   |   |   |   |   |   |   |   | 4
    /// +---+---+---+---+---+---+---+---+
    /// |   |   |   |   |   |   |   |   | 3
    /// +---+---+---+---+---+---+---+---+
    /// | ♟ | ♟ | ♟ | ♟ | ♟ | ♟ | ♟ | ♟ | 2
    /// +---+---+---+---+---+---+---+---+
    /// | ♜ | ♞ | ♝ | ♛ | ♚ | ♝ | ♞ | ♜ | 1
    /// +---+---+---+---+---+---+---+---+
    ///   a   b   c   d   e   f   g   h
    /// ```
    pub fn print(&self) {
        for row in (1..=8).rev() {
            println!("+---+---+---+---+---+---+---+---+");
            print!("|");
            for column in 'a'..='h' {
                if let Some(p) = self.at(Pos::new(column, row)) {
                    let color = match p.p_side() {
                        piece::Side::White => "\x1b[1;37m",
                        piece::Side::Black => "\x1b[1;31m",
                    };
                    print!(" {}{}\x1b[0m |", color, p.p_type());
                } else {
                    print!("   |");
                }
            }
            print!(" {}\n", row);
        }
        println!("+---+---+---+---+---+---+---+---+");

        for column in 'a'..='h' {
            print!("  {} ", column);
        }
    }
}
