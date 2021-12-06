use {
    super::{
        parse_bingo_board_error::ParseBingoBoardError,
        parse_call_sequence_error::ParseCallSequenceError,
    },
    std::{
        error::Error,
        fmt::{Display, Formatter, Result},
    },
};

#[derive(Debug)]
pub enum ReadBingoGameError {
    ParseCallSequenceError(ParseCallSequenceError),
    ParseBingoBoardError(ParseBingoBoardError),
    InsufficientGroups,
}

impl Error for ReadBingoGameError {}

impl Display for ReadBingoGameError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::ParseCallSequenceError(value) => {
                write!(f, "ReadBingoGameError::ParseCallSequenceError({})", value)
            }
            Self::ParseBingoBoardError(value) => {
                write!(f, "ReadBingoGameError::ParseBingoBoardError({})", value)
            }
            Self::InsufficientGroups => write!(f, "ReadBingoGameError::InsufficientGroups"),
        }
    }
}
