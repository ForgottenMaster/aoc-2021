use {
    super::{bingo_board_row::BingoBoardRow, parse_bingo_board_error::ParseBingoBoardError},
    std::str::FromStr,
};

/// Struct that represents a bingo board in the bingo game.
#[derive(Debug, PartialEq)]
pub struct BingoBoard(Vec<BingoBoardRow>);

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
}
