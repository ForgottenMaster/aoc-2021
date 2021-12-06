use std::num::ParseIntError;

/// used as the error type from attempting to parse
/// the point from a string.
#[derive(Debug)]
pub enum ParsePointError {
    ParseIntError(ParseIntError),
    NotEnoughParts,
    TooManyParts,
}

impl From<ParseIntError> for ParsePointError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}
