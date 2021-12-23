use super::Board;

/// Tracks the current position of a single pawn in the game, and the
/// current score.
#[derive(Debug, PartialEq)]
pub struct Pawn {
    position: u8,
    score: u64,
}

impl Pawn {
    /// Creates a new Pawn at the given starting position with a score of
    /// zero.
    pub fn new(position: u8) -> Self {
        Self { position, score: 0 }
    }

    /// Applies the given number of steps to the pawn (using the
    /// given board to determine the landing space). Updates the score
    /// based on the landing position.
    pub fn apply_steps(&mut self, board: &Board, steps: u16) {
        self.position = board.landing_space(self.position, steps);
        self.score += self.position as u64;
    }

    /// Gets the current score
    pub fn score(&self) -> u64 {
        self.score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pawn_new() {
        let expected = Pawn {
            position: 4,
            score: 0,
        };
        let calculated = Pawn::new(4);
        assert_eq!(calculated, expected);
    }

    #[test]
    fn test_pawn_apply_steps() {
        let mut expected = Pawn::new(4);
        let board = Board::new(1, 10);
        expected.apply_steps(&board, 7);
        let calculated = Pawn {
            position: 1,
            score: 1,
        };
        assert_eq!(calculated, expected);
        expected.apply_steps(&board, 23);
        let calculated = Pawn {
            position: 4,
            score: 5,
        };
        assert_eq!(calculated, expected);
    }

    #[test]
    fn test_pawn_score() {
        let calculated = Pawn {
            position: 0,
            score: 42,
        }
        .score();
        let expected = 42;
        assert_eq!(calculated, expected);
    }
}
