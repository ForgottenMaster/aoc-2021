use {
    super::read_diagnostic_report_error::ReadDiagnosticReportError,
    crate::common::binary::{ParseBinaryStringError, ParsedBinaryString},
    std::{
        io::BufRead,
        ops::{Add, Shl},
    },
};

#[derive(Debug, PartialEq)]
pub struct DiagnosticReport<T>(Vec<ParsedBinaryString<T>>);

impl<T> DiagnosticReport<T> {
    pub fn unwrap(self) -> Vec<ParsedBinaryString<T>> {
        self.0
    }
}

impl<T: Add<T, Output = T> + From<bool> + Shl<u8, Output = T>> DiagnosticReport<T> {
    pub fn new_from_bufread(reader: impl BufRead) -> Result<Self, ReadDiagnosticReportError> {
        let mut lines = Vec::new();
        let mut line_len = 0;

        for (line_num, line) in reader.lines().filter_map(|line| line.ok()).enumerate() {
            match line.parse::<ParsedBinaryString<T>>() {
                Err(ParseBinaryStringError::EmptyString) => continue,
                Err(err) => return Err(ReadDiagnosticReportError::ParseBinaryStringError(err)),
                Ok(parsed) => {
                    if line_len != 0 && line_len != parsed.input_string_bit_count() {
                        return Err(ReadDiagnosticReportError::InvalidLineLength {
                            line_num,
                            expected: line_len,
                        });
                    } else {
                        line_len = parsed.input_string_bit_count();
                        lines.push(parsed);
                    }
                }
            }
        }

        Ok(Self(lines))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_diagnostic_report_parse_binary_string_error() {
        assert!(
            match DiagnosticReport::<u32>::new_from_bufread("011o1".as_bytes()) {
                Err(ReadDiagnosticReportError::ParseBinaryStringError(_)) => true,
                _ => false,
            }
        );
    }

    #[test]
    fn test_read_diagnostic_report_invalid_line_length() {
        assert!(match DiagnosticReport::<u32>::new_from_bufread(
            r#"
        
        10010
        001

        "#
            .as_bytes()
        ) {
            Err(ReadDiagnosticReportError::InvalidLineLength {
                line_num: 3,
                expected: 5,
            }) => true,
            _ => false,
        })
    }

    #[test]
    fn test_read_diagnostic_report_success() {
        const INPUT: &[u8] = r#"
        
            01 00 1
        101 00

                0101     1
        "#
        .as_bytes();
        let expected = DiagnosticReport(vec![
            "01001".parse().unwrap(),
            "10100".parse().unwrap(),
            "01011".parse().unwrap(),
        ]);
        let calculated = DiagnosticReport::<u32>::new_from_bufread(INPUT).unwrap();
        assert_eq!(calculated, expected);
    }
}
