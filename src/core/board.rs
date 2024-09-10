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
                fields[Pos::at(column, row)] = Some(Piece::new(piece::Type::Pawn(false), side));
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

    /// Returns the side which should make the next move. (`current_move` field)
    pub fn current_move(&self) -> piece::Side {
        self.current_move
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

    /// Performs a piece movement by first calling `possible_moves(start_pos)` and checking if
    /// `end_pos` is a valid movement. If it isn't, an explanatory `Err` is returned, else
    /// the movement is performed and `Ok` is returned.
    pub fn move_piece(&mut self, start_pos: Pos, end_pos: Pos) -> Result<(), &'static str> {
        if let Some(p) = self.at(start_pos) {
            if p.p_side() != self.current_move {
                return Err("Wrong piece was selected.");
            }
        } else {
            return Err("An empty field was selected.");
        }

        let moves = self.possible_moves(start_pos);
        if moves.contains(&end_pos) {
            self.fields[usize::from(end_pos)] = self.fields[usize::from(start_pos)];
            self.fields[usize::from(start_pos)] = None;

            if let Some(p) = self.at_mut(end_pos) {
                p.mark_moved();
            }

            self.current_move = if self.current_move == piece::Side::White {
                piece::Side::Black
            } else {
                piece::Side::White
            };
            return Ok(());
        } else {
            return Err("Specified move is impossible.");
        }
    }
    /// Returns possible positions for a piece at `pos` as a vector.
    /// The vector is empty if an empty field was selected or when the specified piece
    /// has no possible moves.
    fn possible_moves(&self, pos: Pos) -> Vec<Pos> {
        let mut moves: Vec<Pos> = Vec::new();
        let selected = self.at(pos);

        if let Some(p) = selected {
            match p.p_type() {
                piece::Type::King => {
                    let c1: char = (pos.column as u8 - 1).into();
                    let c2: char = (pos.column as u8 + 1).into();
                    for c in c1..=c2 {
                        for r in pos.row - 1..=pos.row + 1 {
                            let cur = Pos::new(c, r);
                            if cur != pos && cur.is_valid() {
                                match self.at(cur) {
                                    Some(p) => {
                                        if p.p_side() != self.current_move {
                                            moves.push(cur);
                                        }
                                    }
                                    None => moves.push(cur),
                                }
                            }
                        }
                    }
                }
                piece::Type::Queen => {
                    // Rook code
                    for r in pos.row..=8 {
                        let cur = Pos::new(pos.column, r);
                        if let Some(p) = self.at(cur) {
                            if p.p_side() != self.current_move {
                                moves.push(cur);
                            }
                            break;
                        } else {
                            moves.push(cur);
                        }
                    }
                    for r in (1..=pos.row).rev() {
                        let cur = Pos::new(pos.column, r);
                        if let Some(p) = self.at(cur) {
                            if p.p_side() != self.current_move {
                                moves.push(cur);
                            }
                            break;
                        } else {
                            moves.push(cur);
                        }
                    }
                    for c in pos.column..='h' {
                        let cur = Pos::new(c, pos.row);
                        if let Some(p) = self.at(cur) {
                            if p.p_side() != self.current_move {
                                moves.push(cur);
                            }
                            break;
                        } else {
                            moves.push(cur);
                        }
                    }
                    for c in ('a'..=pos.column).rev() {
                        let cur = Pos::new(c, pos.row);
                        if let Some(p) = self.at(cur) {
                            if p.p_side() != self.current_move {
                                moves.push(cur);
                            }
                            break;
                        } else {
                            moves.push(cur);
                        }
                    }
                    // Bishop code
                    for i in 1.. {
                        let cur = Pos::new((pos.column as u8 + i).into(), pos.row + i);
                        if !cur.is_valid() {
                            break;
                        }
                        if let Some(p) = self.at(cur) {
                            if p.p_side() != self.current_move {
                                moves.push(cur);
                            }
                            break;
                        } else {
                            moves.push(cur);
                        }
                    }
                    for i in 1.. {
                        let cur = Pos::new((pos.column as u8 + i).into(), pos.row - i);
                        if !cur.is_valid() {
                            break;
                        }
                        if let Some(p) = self.at(cur) {
                            if p.p_side() != self.current_move {
                                moves.push(cur);
                            }
                            break;
                        } else {
                            moves.push(cur);
                        }
                    }
                    for i in 1.. {
                        let cur = Pos::new((pos.column as u8 - i).into(), pos.row - i);
                        if !cur.is_valid() {
                            break;
                        }
                        if let Some(p) = self.at(cur) {
                            if p.p_side() != self.current_move {
                                moves.push(cur);
                            }
                            break;
                        } else {
                            moves.push(cur);
                        }
                    }
                    for i in 1.. {
                        let cur = Pos::new((pos.column as u8 - i).into(), pos.row + i);
                        if !cur.is_valid() {
                            break;
                        }
                        if let Some(p) = self.at(cur) {
                            if p.p_side() != self.current_move {
                                moves.push(cur);
                            }
                            break;
                        } else {
                            moves.push(cur);
                        }
                    }
                }
                piece::Type::Rook => {
                    for r in pos.row..=8 {
                        let cur = Pos::new(pos.column, r);
                        if let Some(p) = self.at(cur) {
                            if p.p_side() != self.current_move {
                                moves.push(cur);
                            }
                            break;
                        } else {
                            moves.push(cur);
                        }
                    }
                    for r in (1..=pos.row).rev() {
                        let cur = Pos::new(pos.column, r);
                        if let Some(p) = self.at(cur) {
                            if p.p_side() != self.current_move {
                                moves.push(cur);
                            }
                            break;
                        } else {
                            moves.push(cur);
                        }
                    }
                    for c in pos.column..='h' {
                        let cur = Pos::new(c, pos.row);
                        if let Some(p) = self.at(cur) {
                            if p.p_side() != self.current_move {
                                moves.push(cur);
                            }
                            break;
                        } else {
                            moves.push(cur);
                        }
                    }
                    for c in ('a'..=pos.column).rev() {
                        let cur = Pos::new(c, pos.row);
                        if let Some(p) = self.at(cur) {
                            if p.p_side() != self.current_move {
                                moves.push(cur);
                            }
                            break;
                        } else {
                            moves.push(cur);
                        }
                    }
                }
                piece::Type::Bishop => {
                    for i in 1.. {
                        let cur = Pos::new((pos.column as u8 + i).into(), pos.row + i);
                        if !cur.is_valid() {
                            break;
                        }
                        if let Some(p) = self.at(cur) {
                            if p.p_side() != self.current_move {
                                moves.push(cur);
                            }
                            break;
                        } else {
                            moves.push(cur);
                        }
                    }
                    for i in 1.. {
                        let cur = Pos::new((pos.column as u8 + i).into(), pos.row - i);
                        if !cur.is_valid() {
                            break;
                        }
                        if let Some(p) = self.at(cur) {
                            if p.p_side() != self.current_move {
                                moves.push(cur);
                            }
                            break;
                        } else {
                            moves.push(cur);
                        }
                    }
                    for i in 1.. {
                        let cur = Pos::new((pos.column as u8 - i).into(), pos.row - i);
                        if !cur.is_valid() {
                            break;
                        }
                        if let Some(p) = self.at(cur) {
                            if p.p_side() != self.current_move {
                                moves.push(cur);
                            }
                            break;
                        } else {
                            moves.push(cur);
                        }
                    }
                    for i in 1.. {
                        let cur = Pos::new((pos.column as u8 - i).into(), pos.row + i);
                        if !cur.is_valid() {
                            break;
                        }
                        if let Some(p) = self.at(cur) {
                            if p.p_side() != self.current_move {
                                moves.push(cur);
                            }
                            break;
                        } else {
                            moves.push(cur);
                        }
                    }
                }
                piece::Type::Knight => {
                    let p = [
                        Pos::new((pos.column as u8 + 1).into(), pos.row + 2),
                        Pos::new((pos.column as u8 + 2).into(), pos.row + 1),
                        Pos::new((pos.column as u8 + 2).into(), pos.row - 1),
                        Pos::new((pos.column as u8 + 1).into(), pos.row - 2),
                        Pos::new((pos.column as u8 - 1).into(), pos.row + 2),
                        Pos::new((pos.column as u8 - 2).into(), pos.row + 1),
                        Pos::new((pos.column as u8 - 2).into(), pos.row - 1),
                        Pos::new((pos.column as u8 - 1).into(), pos.row - 2),
                    ];

                    for cur in p {
                        if cur.is_valid() {
                            if let Some(p) = self.at(cur) {
                                if p.p_side() != self.current_move {
                                    moves.push(cur);
                                }
                            } else {
                                moves.push(cur);
                            }
                        }
                    }
                }
                piece::Type::Pawn(has_moved) => {
                    let cur_relative = |c: u8, r: u8, subtract: bool| {
                        let mut cur = match self.current_move {
                            piece::Side::White => Pos::new(pos.column, pos.row + r),
                            piece::Side::Black => Pos::new(pos.column, pos.row - r),
                        };
                        cur.column = if subtract {
                            (cur.column as u8 - c).into()
                        } else {
                            (cur.column as u8 + c).into()
                        };
                        cur
                    };

                    let cur = cur_relative(0, 1, false);
                    if cur.is_valid() {
                        if let None = self.at(cur) {
                            moves.push(cur);
                        }
                    }
                    let cur = cur_relative(1, 1, true);
                    if cur.is_valid() {
                        if let Some(p) = self.at(cur) {
                            if p.p_side() != self.current_move {
                                moves.push(cur);
                            }
                        }
                    }
                    let cur = cur_relative(1, 1, false);
                    if cur.is_valid() {
                        if let Some(p) = self.at(cur) {
                            if p.p_side() != self.current_move {
                                moves.push(cur);
                            }
                        }
                    }
                    if !has_moved {
                        let cur = cur_relative(0, 2, false);
                        if cur.is_valid() {
                            if let None = self.at(cur) {
                                moves.push(cur);
                            }
                        }
                    }
                }
            }
        } else {
            return vec![];
        }

        moves
    }
}
