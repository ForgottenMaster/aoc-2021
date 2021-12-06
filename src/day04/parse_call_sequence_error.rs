use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
    num::ParseIntError,
};

#[derive(Debug)]
pub enum ParseCallSequenceError {
    ParseIntError(ParseIntError),
    EmptySequence,
}

impl Error for ParseCallSequenceError {}

impl Display for ParseCallSequenceError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::ParseIntError(value) => {
                write!(f, "ParseCallSequenceError::ParseIntError({})", value)
            }
            Self::EmptySequence => write!(f, "ParseCallSequenceError::EmptySequence"),
        }
    }
}
