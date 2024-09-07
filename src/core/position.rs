#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Pos {
    pub column: char,
    pub row: u8,
}

impl Pos {
    pub fn new(column: char, row: u8) -> Self {
        Self { column, row }
    }
    pub fn at(column: char, row: u8) -> usize {
        (column as u8 - 'a' as u8 + (row - 1) * 8).into()
    }
    pub fn get(&self) -> usize {
        (self.column as u8 - 'a' as u8 + (self.row - 1) * 8).into()
    }
}

impl From<Pos> for usize {
    fn from(value: Pos) -> Self {
        (value.column as u8 - 'a' as u8 + (value.row - 1) * 8).into()
    }
}
impl From<usize> for Pos {
    fn from(value: usize) -> Self {
        todo!()
    }
}
