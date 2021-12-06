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

    pub fn try_call_number(&mut self, number: u32) -> bool {
        (&mut self.0)
            .into_iter()
            .any(|entry| entry.try_call_number(number))
    }
}

impl<'a> IntoIterator for &'a BingoBoardRow {
    type Item = &'a BingoBoardEntry;
    type IntoIter = <&'a [BingoBoardEntry] as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        (&self.0 as &[BingoBoardEntry]).into_iter()
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

    #[test]
    fn test_bingo_board_row_len() {
        assert_eq!(
            BingoBoardRow(vec![BingoBoardEntry::new(12), BingoBoardEntry::new(23)]).len(),
            2
        );
    }

    #[test]
    fn test_bingo_board_row_borrowed_into_iter() {
        let bingo_board_row =
            BingoBoardRow(vec![BingoBoardEntry::new(12), BingoBoardEntry::new(24)]);
        assert_eq!(bingo_board_row.into_iter().count(), 2);
        assert_eq!(bingo_board_row.into_iter().count(), 2); // test can use multiple times
    }

    #[test]
    fn test_bingo_board_row_try_call_number_invalid() {
        let mut bingo_board_row =
            BingoBoardRow(vec![BingoBoardEntry::new(1), BingoBoardEntry::new(2)]);
        assert!(!bingo_board_row.try_call_number(10));
    }

    #[test]
    fn test_bingo_board_row_try_call_number_valid() {
        let mut bingo_board_row =
            BingoBoardRow(vec![BingoBoardEntry::new(1), BingoBoardEntry::new(2)]);
        assert!(bingo_board_row.try_call_number(2));
        assert!(bingo_board_row
            .0
            .into_iter()
            .skip(1)
            .next()
            .unwrap()
            .is_marked());
    }
}
