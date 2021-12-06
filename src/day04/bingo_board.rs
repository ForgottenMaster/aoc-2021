use {
    super::{
        bingo_board_row::BingoBoardRow, call_sequence::CallSequence,
        parse_bingo_board_error::ParseBingoBoardError,
    },
    std::str::FromStr,
};

/// Struct that represents a bingo board in the bingo game.
#[derive(Debug, PartialEq)]
pub struct BingoBoard(Vec<BingoBoardRow>);

impl BingoBoard {
    pub fn is_winning_board(&self) -> bool {
        self.has_complete_row() || self.has_complete_column()
    }

    fn has_complete_row(&self) -> bool {
        (&self.0)
            .into_iter()
            .any(|row| row.into_iter().all(|entry| entry.is_marked()))
    }

    fn has_complete_column(&self) -> bool {
        if let Some(row) = (&self.0).into_iter().next() {
            (0..row.len()).into_iter().any(|idx| {
                (&self.0).into_iter().all(|row| {
                    if let Some(row_entry) = row.into_iter().skip(idx).next() {
                        row_entry.is_marked()
                    } else {
                        false
                    }
                })
            })
        } else {
            false
        }
    }

    fn try_call_number(&mut self, number: u32) -> bool {
        (&mut self.0)
            .into_iter()
            .any(|row| row.try_call_number(number))
    }

    // Returns the number of numbers called before a winner is found.
    pub fn run_call_sequence(&mut self, call_sequence: &CallSequence) -> Option<u32> {
        let mut idx = 0;
        for number in call_sequence {
            idx += 1;
            if self.try_call_number(*number) && self.is_winning_board() {
                return Some(idx);
            }
        }
        None
    }

    pub fn sum_unmarked(&self) -> u32 {
        (&self.0)
            .into_iter()
            .flat_map(|row| row.into_iter())
            .filter_map(|entry| {
                if !entry.is_marked() {
                    Some(entry.value())
                } else {
                    None
                }
            })
            .sum()
    }
}

impl FromStr for BingoBoard {
    type Err = ParseBingoBoardError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut rows = Vec::new();
        let mut row_length = 0;

        for row in string
            .trim()
            .lines()
            .map(|line| line.trim().parse::<BingoBoardRow>())
        {
            let row = row.map_err(|err| ParseBingoBoardError::BingoBoardRowError(err))?;
            if row_length != 0 && row.len() != row_length {
                return Err(ParseBingoBoardError::MismatchingRowLengths);
            }
            row_length = row.len();
            rows.push(row);
        }

        if rows.is_empty() {
            Err(ParseBingoBoardError::EmptyBoard)
        } else {
            Ok(Self(rows))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bingo_board_empty() {
        const INPUT: &str = "        ";
        assert!(match INPUT.parse::<BingoBoard>() {
            Err(ParseBingoBoardError::EmptyBoard) => true,
            _ => false,
        });
    }

    #[test]
    fn test_bingo_board_mismatching_rows() {
        const INPUT: &str = r#"
        1 2 3 4
        1 2 3
        "#;
        assert!(match INPUT.parse::<BingoBoard>() {
            Err(ParseBingoBoardError::MismatchingRowLengths) => true,
            _ => false,
        });
    }

    #[test]
    fn test_bingo_board_single_row() {
        const INPUT: &str = "   1 2  3    4    ";
        let expected = BingoBoard(vec![INPUT.parse::<BingoBoardRow>().unwrap()]);
        let calculated = INPUT.parse::<BingoBoard>().unwrap();
        assert_eq!(expected, calculated);
    }

    #[test]
    fn test_bingo_board_try_call_number_invalid() {
        let mut bingo_board = r#"
        1 2 3
        4 5 6
        7 8 9
        "#
        .parse::<BingoBoard>()
        .unwrap();
        assert!(!bingo_board.try_call_number(11));
    }

    #[test]
    fn test_bingo_board_try_call_number_valid() {
        let mut bingo_board = r#"
        1 2 3
        4 5 6
        7 8 9
        "#
        .parse::<BingoBoard>()
        .unwrap();
        assert!(bingo_board.try_call_number(4));
        assert!(bingo_board
            .0
            .into_iter()
            .skip(1)
            .next()
            .unwrap()
            .into_iter()
            .next()
            .unwrap()
            .is_marked()); // assert it was actually marked
    }

    #[test]
    fn test_bingo_board_incomplete_row() {
        let mut bingo_board = r#"
        1 2 3
        4 5 6
        7 8 9
        "#
        .parse::<BingoBoard>()
        .unwrap();
        bingo_board.try_call_number(1);
        bingo_board.try_call_number(2);
        assert!(!bingo_board.has_complete_row());
    }

    #[test]
    fn test_bingo_board_complete_row() {
        let mut bingo_board = r#"
        1 2 3
        4 5 6
        7 8 9
        "#
        .parse::<BingoBoard>()
        .unwrap();
        bingo_board.try_call_number(4);
        bingo_board.try_call_number(5);
        bingo_board.try_call_number(6);
        assert!(bingo_board.has_complete_row());
    }

    #[test]
    fn test_bingo_board_incomplete_column() {
        let mut bingo_board = r#"
        1 2 3
        4 5 6
        7 8 9
        "#
        .parse::<BingoBoard>()
        .unwrap();
        bingo_board.try_call_number(1);
        bingo_board.try_call_number(2);
        assert!(!bingo_board.has_complete_column());
    }

    #[test]
    fn test_bingo_board_complete_column() {
        let mut bingo_board = r#"
        1 2 3
        4 5 6
        7 8 9
        "#
        .parse::<BingoBoard>()
        .unwrap();
        bingo_board.try_call_number(1);
        bingo_board.try_call_number(4);
        bingo_board.try_call_number(7);
        assert!(bingo_board.has_complete_column());
    }

    #[test]
    fn test_bingo_board_completed_with_row() {
        let mut bingo_board = r#"
        1 2 3
        4 5 6
        7 8 9
        "#
        .parse::<BingoBoard>()
        .unwrap();
        bingo_board.try_call_number(4);
        bingo_board.try_call_number(5);
        bingo_board.try_call_number(6);
        assert!(bingo_board.is_winning_board());
    }

    #[test]
    fn test_bingo_board_completed_with_column() {
        let mut bingo_board = r#"
        1 2 3
        4 5 6
        7 8 9
        "#
        .parse::<BingoBoard>()
        .unwrap();
        bingo_board.try_call_number(2);
        bingo_board.try_call_number(5);
        bingo_board.try_call_number(8);
        assert!(bingo_board.is_winning_board());
    }

    #[test]
    fn test_bingo_board_sum_unmarked() {
        let mut bingo_board = r#"
        1 2 3
        4 5 6
        7 8 9
        "#
        .parse::<BingoBoard>()
        .unwrap();
        let call_sequence = "1, 6, 7".parse::<CallSequence>().unwrap();
        bingo_board.run_call_sequence(&call_sequence);
        assert_eq!(bingo_board.sum_unmarked(), 31);
    }
}
