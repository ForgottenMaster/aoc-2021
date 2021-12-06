use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
    num::ParseIntError,
};

#[derive(Debug)]
pub enum ParseBingoBoardRowError {
    ParseIntError(ParseIntError),
    EmptySequence,
}

impl Error for ParseBingoBoardRowError {}

impl Display for ParseBingoBoardRowError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::ParseIntError(value) => {
                write!(f, "ParseBingoBoardRowError::ParseIntError({})", value)
            }
            Self::EmptySequence => write!(f, "ParseBingoBoardRowError::EmptySequence"),
        }
    }
}
