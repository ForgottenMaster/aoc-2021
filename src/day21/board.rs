/// Represents the board for the game. Has a minimum and maximum board number
/// and allows for movement around the board by the given number of steps.
#[derive(Debug, PartialEq)]
pub struct Board {
    min: u8,
    max: u8,
}

impl Board {
    pub fn new(min: u8, max: u8) -> Self {
        Self { min, max }
    }

    /// Calculates the final landing space if starting from a given
    /// space and taking the number of steps. Will loop around from max
    /// to min if steps take it over.
    pub fn landing_space(&self, current: u8, steps: u16) -> u8 {
        let modulus = (self.max - self.min + 1) as u16;
        let current = current - self.min;
        let landing_mod = (current as u16 + steps) % modulus;
        landing_mod as u8 + self.min
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_new() {
        let expected = Board { min: 1, max: 10 };
        let calculated = Board::new(1, 10);
        assert_eq!(calculated, expected);
    }

    #[test]
    fn test_board_landing_space() {
        let board = Board::new(1, 10);
        assert_eq!(board.landing_space(6, 8), 4);
        assert_eq!(board.landing_space(7, 2), 9);
        assert_eq!(board.landing_space(9, 2), 1);
        let board = Board::new(3, 7);
        assert_eq!(board.landing_space(3, 0), 3);
        assert_eq!(board.landing_space(5, 27), 7);
    }
}
