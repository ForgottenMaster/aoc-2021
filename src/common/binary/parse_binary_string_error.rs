use std::fmt::{Display, Formatter, Result};

/// Error cases for the binary string parsing.
#[derive(Debug)]
pub enum ParseBinaryStringError {
    EmptyString,
    InvalidChar {
        string: String,
        index: usize,
        character: char,
    },
}

impl Display for ParseBinaryStringError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::EmptyString => write!(f, "ParseBinaryStringError::EmptyString"),
            Self::InvalidChar {
                string,
                index,
                character,
            } => write!(
                f,
                "ParseBinaryStringError::InvalidChar {{ string: {}, index: {}, character: {} }}",
                string, index, character
            ),
        }
    }
}
