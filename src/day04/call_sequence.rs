use {super::parse_call_sequence_error::ParseCallSequenceError, std::str::FromStr};

/// Struct that represents the call sequence for the bingo game.
#[derive(Debug, PartialEq)]
pub struct CallSequence(Vec<u32>);

impl FromStr for CallSequence {
    type Err = ParseCallSequenceError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut sequence = Vec::new();
        for value in string
            .trim()
            .split(",")
            .map(|elem| elem.trim())
            .filter(|elem| !elem.is_empty())
            .map(|elem| elem.parse())
        {
            sequence.push(value.map_err(|err| ParseCallSequenceError::ParseIntError(err))?);
        }

        if sequence.is_empty() {
            Err(ParseCallSequenceError::EmptySequence)
        } else {
            Ok(Self(sequence))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_call_sequence_invalid_char() {
        const INPUT: &str = "21,45,i5";
        assert!(match INPUT.parse::<CallSequence>() {
            Err(ParseCallSequenceError::ParseIntError(_)) => true,
            _ => false,
        });
    }

    #[test]
    fn test_call_sequence_empty() {
        const INPUT: &str = "     ";
        assert!(match INPUT.parse::<CallSequence>() {
            Err(ParseCallSequenceError::EmptySequence) => true,
            _ => false,
        });
    }

    #[test]
    fn test_call_sequence_valid() {
        const INPUT: &str = "   42,  365, 18, 24    ";
        let expected = vec![42, 365, 18, 24];
        let calculated = INPUT.parse::<CallSequence>().unwrap().0;
        assert_eq!(calculated, expected);
    }
}
