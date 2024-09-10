use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum PosParseError {
    InvalidPos,
    InsufficientArgs,
}

impl fmt::Display for PosParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PosParseError::InvalidPos => write!(f, "invalid position specified"),
            PosParseError::InsufficientArgs => write!(f, "insufficient arguments provided"),
        }
    }
}

impl Error for PosParseError {}

#[derive(Debug)]
pub struct QuitGame;

impl fmt::Display for QuitGame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "quitting game")
    }
}

impl Error for QuitGame {}
