use std::{char, error::Error};
use std::io;
use std::io::Read;

use crate::core::position::Pos;

use super::error::{PosParseError, QuitGame};

pub struct InputPair(pub Pos, pub Pos);

pub fn query_input() -> Result<InputPair, Box<dyn Error>> {
    println!("Select piece (example: d2), 'qq' - quits:");
    let p1 = {
        let mut buf = [0; 2];
        let mut input = io::BufReader::new(io::stdin());
        input.read_exact(&mut buf)?;
        parse_pos(&buf)?
    };

    println!("Select move (example: d4), 'qq' - quits:");
    let p2 = {
        let mut buf = [0; 2];
        let mut input = io::BufReader::new(io::stdin());
        input.read_exact(&mut buf)?;
        parse_pos(&buf)?
    };

    Ok(InputPair(p1, p2))
}

fn parse_pos(buf: &[u8; 2]) -> Result<Pos, Box<dyn Error>> {
    let mut i = buf.iter();

    let column: char;
    match i.next() {
        Some(c) => column = *c as char,
        None => return Err(Box::new(PosParseError::InsufficientArgs)),
    }

    let row: u8;
    match i.next() {
        Some(r) => row = *r - b'0',
        None => return Err(Box::new(PosParseError::InsufficientArgs)),
    }

    if column == 'q' && row == b'q' - b'0' {
        return Err(Box::new(QuitGame));
    }

    let pos = Pos::new(column, row);
    if !pos.is_valid() {
        Err(Box::new(PosParseError::InvalidPos))
    } else {
        Ok(pos)
    }
}
