/// Struct defining a single entry in the bingo board.
#[derive(Debug, PartialEq)]
pub struct BingoBoardEntry {
    value: u32,
    is_marked: bool,
}

impl BingoBoardEntry {
    pub fn new(value: u32) -> Self {
        Self {
            value,
            is_marked: false,
        }
    }
}
