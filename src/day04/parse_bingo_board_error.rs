use {
    super::parse_bingo_board_row_error::ParseBingoBoardRowError,
    std::{
        error::Error,
        fmt::{Display, Formatter, Result},
    },
};

#[derive(Debug)]
pub enum ParseBingoBoardError {
    BingoBoardRowError(ParseBingoBoardRowError),
    EmptyBoard,
    MismatchingRowLengths,
}

impl Error for ParseBingoBoardError {}

impl Display for ParseBingoBoardError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::BingoBoardRowError(value) => {
                write!(f, "ParseBingoBoardError::BingoBoardRowError({})", value)
            }
            Self::EmptyBoard => write!(f, "ParseBingoBoardError::EmptyBoard"),
            Self::MismatchingRowLengths => {
                write!(f, "ParseBingoBoardError::MismatchingRowLengths")
            }
        }
    }
}
