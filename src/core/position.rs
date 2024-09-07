#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Pos {
    pub column: char,
    pub row: u8,
}

impl Pos {
    /// Constructs a new position (`Pos`) from a given column and row
    /// If the column is not in range from 'a' to 'h' (inclusive)
    /// or the row is not in range from 1 to 8 (inclusive)
    /// the position struct is ill-formed and behaviour is undefined
    pub fn new(column: char, row: u8) -> Self {
        Self { column, row }
    }
    /// Associated function of position (`Pos`) equivalent to
    /// ```
    /// Pos::new(column, row).get()
    /// ```
    /// 
    /// If the column is not in range from 'a' to 'h' (inclusive)
    /// or the row is not in range from 1 to 8 (inclusive)
    /// the behaviour is undefined
    ///
    /// # Example
    ///
    /// ```
    /// let p1 = Pos::new('d', 4);
    ///
    /// assert_eq!(p1.get(), Pos::at('d', 4));
    /// ```
    pub fn at(column: char, row: u8) -> usize {
        (column as u8 - 'a' as u8 + (row - 1) * 8).into()
    }
    /// Returns an index to a 64 element slice at the position stored in `&self`.
    /// If the position is ill-formed this method might or might not panic.
    pub fn get(&self) -> usize {
        (self.column as u8 - 'a' as u8 + (self.row - 1) * 8).into()
    }
}

/// Can be used interchangeably with `Pos::get()`
impl From<Pos> for usize {
    fn from(value: Pos) -> Self {
        (value.column as u8 - 'a' as u8 + (value.row - 1) * 8).into()
    }
}
