use std::fmt;

#[derive(Clone, Copy)]
/// Used to store a chess piece and its data
pub struct Piece {
    p_type: Type,
    p_side: Side,
}

#[derive(Clone, Copy)]
/// Type of a chess piece
pub enum Type {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Clone, Copy)]
/// Side the chess piece is on
pub enum Side {
    White,
    Black,
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
            Type::Pawn => '♟',
        };
        write!(f, "{c}")
    }
}
