use {
    super::{
        bingo_board_entry::BingoBoardEntry, parse_bingo_board_row_error::ParseBingoBoardRowError,
    },
    std::str::FromStr,
};

/// Struct that represents a single row in the bingo game.
#[derive(Debug, PartialEq)]
pub struct BingoBoardRow(Vec<BingoBoardEntry>);

impl BingoBoardRow {
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl FromStr for BingoBoardRow {
    type Err = ParseBingoBoardRowError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut sequence = Vec::new();
        for value in string
            .trim()
            .split(" ")
            .map(|elem| elem.trim())
            .filter(|elem| !elem.is_empty())
            .map(|elem| elem.parse())
        {
            sequence.push(BingoBoardEntry::new(
                value.map_err(|err| ParseBingoBoardRowError::ParseIntError(err))?,
            ));
        }

        if sequence.is_empty() {
            Err(ParseBingoBoardRowError::EmptySequence)
        } else {
            Ok(Self(sequence))
        }
    }
}

#[cfg(test)]
mod tests {
    use {super::super::parse_bingo_board_row_error::ParseBingoBoardRowError, super::*};

    #[test]
    fn test_bingo_board_row_invalid_char() {
        const INPUT: &str = "21 45 i5";
        assert!(match INPUT.parse::<BingoBoardRow>() {
            Err(ParseBingoBoardRowError::ParseIntError(_)) => true,
            _ => false,
        });
    }

    #[test]
    fn test_bingo_board_row_empty() {
        const INPUT: &str = "     ";
        assert!(match INPUT.parse::<BingoBoardRow>() {
            Err(ParseBingoBoardRowError::EmptySequence) => true,
            _ => false,
        });
    }

    #[test]
    fn test_bingo_board_row_valid() {
        const INPUT: &str = "   42  365  18  24    ";
        let expected = vec![
            BingoBoardEntry::new(42),
            BingoBoardEntry::new(365),
            BingoBoardEntry::new(18),
            BingoBoardEntry::new(24),
        ];
        let calculated = INPUT.parse::<BingoBoardRow>().unwrap().0;
        assert_eq!(calculated, expected);
    }
}
