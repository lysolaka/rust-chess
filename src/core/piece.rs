use std::fmt;

#[derive(Clone, Copy)]
/// Used to store a chess piece and its data
pub struct Piece {
    p_type: Type,
    p_side: Side,
}

#[derive(PartialEq, Clone, Copy)]
/// Type of a chess piece.
/// Pawns also hold a boolean indicating whether it was moved.
pub enum Type {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn(bool),
}

#[derive(PartialEq, Clone, Copy)]
/// Side the chess piece is on
pub enum Side {
    White,
    Black,
}

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Side::White => write!(f, "White"),
            Side::Black => write!(f, "Black"),
        }
    }
}

impl Piece {
    /// Construct a piece from a given side and type
    pub fn new(p_type: Type, p_side: Side) -> Self {
        Self { p_type, p_side }
    }
    pub fn p_type(&self) -> Type {
        self.p_type
    }
    pub fn p_side(&self) -> Side {
        self.p_side
    }

    /// Marks a pawn as moved by changing its internal boolean to true.
    /// If `self.p_type` is not a pawn this method does nothing.
    pub fn mark_moved(&mut self) {
        if self.p_type == Type::Pawn(false) {
            self.p_type = Type::Pawn(true);
        }
    }
}

impl fmt::Display for Type {
    #[cfg(feature = "ascii")]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Type::King => 'K',
            Type::Queen => 'Q',
            Type::Rook => 'R',
            Type::Bishop => 'B',
            Type::Knight => 'k',
            Type::Pawn => 'p',
        };
        write!(f, "{c}")
    }

    #[cfg(feature = "unicode")]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Type::King => '♚',
            Type::Queen => '♛',
            Type::Rook => '♜',
            Type::Bishop => '♝',
            Type::Knight => '♞',
            Type::Pawn(_) => '♟',
        };
        write!(f, "{c}")
    }
}
