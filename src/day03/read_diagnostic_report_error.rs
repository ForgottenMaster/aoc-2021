use {
    crate::common::binary::ParseBinaryStringError,
    std::{
        error::Error,
        fmt::{Display, Formatter, Result},
    },
};

#[derive(Debug)]
pub enum ReadDiagnosticReportError {
    ParseBinaryStringError(ParseBinaryStringError),
    InvalidLineLength { line_num: usize, expected: usize },
}

impl Display for ReadDiagnosticReportError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::ParseBinaryStringError(value) => write!(
                f,
                "ReadDiagnosticReportError::ParseBinaryStringError({})",
                value
            ),
            Self::InvalidLineLength { line_num, expected } => write!(
                f,
                "ReadDiagnosticReportError::InvalidLineLength {{ line_num: {}, expected: {} }}",
                line_num, expected
            ),
        }
    }
}

impl Error for ReadDiagnosticReportError {}
