use super::parse_point_error::ParsePointError;

/// Enum with the error conditions for parsing a line from string
#[derive(Debug)]
pub enum ParseLineError {
    ParsePointError(ParsePointError),
    NotEnoughParts,
    TooManyParts,
}

impl From<ParsePointError> for ParseLineError {
    fn from(value: ParsePointError) -> Self {
        Self::ParsePointError(value)
    }
}
