/// Struct defining a single entry in the bingo board.
#[derive(Clone, Debug, PartialEq)]
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

    pub fn value(&self) -> u32 {
        self.value
    }

    pub fn is_marked(&self) -> bool {
        self.is_marked
    }

    pub fn try_call_number(&mut self, number: u32) -> bool {
        if self.value == number {
            self.is_marked = true;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bingo_board_entry_try_call_number_invalid() {
        let mut entry = BingoBoardEntry::new(1);
        assert!(!entry.try_call_number(2));
    }

    #[test]
    fn test_bingo_board_entry_try_call_number_valid() {
        let mut entry = BingoBoardEntry::new(1);
        assert!(entry.try_call_number(1));
        assert!(entry.is_marked);
    }
}
