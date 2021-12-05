use {
    aoc_2021::{ParseBinaryStringError, ParsedBinaryString},
    std::{
        fs::File,
        io::{BufRead, BufReader},
        iter::repeat,
        ops::{Add, Shl},
        time::Instant,
    },
};

fn main() -> Result<(), SolutionError> {
    let start_time = Instant::now();
    let file = File::open("input/day03.txt").map_err(|err| SolutionError::IOError(err))?;
    let reader = BufReader::new(file);
    let diagnostic_report = DiagnosticReport::<u32>::new_from_bufread(reader)
        .map_err(|err| SolutionError::ReadDiagnosticReportError(err))?
        .0;
    let bit_count = if diagnostic_report.len() > 0 {
        diagnostic_report[0].input_string_bit_count()
    } else {
        0
    };
    let diagnostic_report = diagnostic_report
        .into_iter()
        .map(|elem| *elem.parsed())
        .collect::<Vec<_>>();

    // Part 1 - need gamma and epsilon rates from the diagnostic report
    {
        let bit_counts = calculate_bit_counts(&diagnostic_report, bit_count);
        let gamma_rate = extract_gamma_rate(&bit_counts);
        let epsilon_rate = extract_epsilon_rate(&bit_counts);
        println!("Part 1 => {}", gamma_rate * epsilon_rate);
    }

    // Part 2 - need oxygen generator rating and co2 scrubber rating
    {
        let oxygen_generator_rating =
            extract_oxygen_generator_rating(diagnostic_report.clone(), bit_count);
        let co2_scrubber_rating = extract_co2_scrubber_rating(diagnostic_report, bit_count);
        println!(
            "Part 2 => {}",
            oxygen_generator_rating * co2_scrubber_rating
        );
    }

    let end_time = Instant::now();
    let duration = end_time.duration_since(start_time);
    println!("Took {} microseconds to run", duration.as_micros());
    Ok(())
}

fn extract_oxygen_generator_rating(mut report: Vec<u32>, bit_count: usize) -> u32 {
    let mut idx = 0;
    while report.len() > 1 {
        let (count_0, count_1) = calculate_bit_counts(&report, bit_count)[idx];
        let most_used_bit = (count_1 >= count_0) as u32;
        report.retain(|elem| {
            let shift = bit_count - 1 - idx;
            ((elem & (1 << shift)) >> shift) == most_used_bit
        });
        idx += 1;
    }
    report[0]
}

fn extract_co2_scrubber_rating(mut report: Vec<u32>, bit_count: usize) -> u32 {
    let mut idx = 0;
    while report.len() > 1 {
        let (count_0, count_1) = calculate_bit_counts(&report, bit_count)[idx];
        let least_used_bit = (!(count_0 <= count_1)) as u32;
        report.retain(|elem| {
            let shift = bit_count - 1 - idx;
            ((elem & (1 << shift)) >> shift) == least_used_bit
        });
        idx += 1;
    }
    report[0]
}

/// Calculates the bit counts for the given report list.
fn calculate_bit_counts(report: &[u32], bit_count: usize) -> Vec<(u32, u32)> {
    report.into_iter().fold(
        repeat((0, 0)).take(bit_count).collect::<Vec<_>>(),
        |mut state, elem| {
            (0..bit_count).for_each(|i| {
                let (mut count_0, mut count_1) = state[i];
                let shift = bit_count - 1 - i;
                if (elem & (1 << shift)) >> shift == 0 {
                    count_0 += 1;
                } else {
                    count_1 += 1;
                }
                state[i] = (count_0, count_1);
            });
            state
        },
    )
}

/// Given the list of 0/1 counts, produces the gamma rate for it.
/// This consists of the most common bits in each position.
fn extract_gamma_rate(report: &[(u32, u32)]) -> u32 {
    let bit_count = report.len();
    report
        .into_iter()
        .enumerate()
        .map(|(idx, (count_0, count_1))| {
            let shift = bit_count - 1 - idx;
            let bit = (count_1 > count_0) as u32;
            bit << shift
        })
        .sum()
}

/// Given the list of 0/1 counts, produces the epsilon rate for it.
/// This consists of the least common bits in each position.
fn extract_epsilon_rate(report: &[(u32, u32)]) -> u32 {
    let bit_count = report.len();
    report
        .into_iter()
        .enumerate()
        .map(|(idx, (count_0, count_1))| {
            let shift = bit_count - 1 - idx;
            let bit = (count_1 < count_0) as u32;
            bit << shift
        })
        .sum()
}

/////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
enum SolutionError {
    IOError(std::io::Error),
    ReadDiagnosticReportError(ReadDiagnosticReportError),
}

/////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq)]
struct DiagnosticReport<T>(Vec<ParsedBinaryString<T>>);

impl<T: Add<T, Output = T> + From<bool> + Shl<u8, Output = T>> DiagnosticReport<T> {
    fn new_from_bufread(reader: impl BufRead) -> Result<Self, ReadDiagnosticReportError> {
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

#[derive(Debug)]
enum ReadDiagnosticReportError {
    ParseBinaryStringError(ParseBinaryStringError),
    InvalidLineLength { line_num: usize, expected: usize },
}

/////////////////////////////////////////////////////////////////////////////////////////////////////

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

    #[test]
    fn test_calculate_bit_counts() {
        const INPUT: &[u32] = &[5, 3, 1, 5];
        const EXPECTED: &[(u32, u32)] = &[(2, 2), (3, 1), (0, 4)];
        let calculated = calculate_bit_counts(INPUT, 3);
        assert_eq!(&calculated, EXPECTED);
    }

    #[test]
    fn test_gamma_rate_calculation() {
        const INPUT: &[(u32, u32)] = &[(5, 7), (7, 5), (4, 8), (5, 7), (7, 5)];
        const EXPECTED: u32 = 22;
        let calculated = extract_gamma_rate(INPUT);
        assert_eq!(calculated, EXPECTED);
    }

    #[test]
    fn test_epsilon_rate_calculation() {
        const INPUT: &[(u32, u32)] = &[(5, 7), (7, 5), (4, 8), (5, 7), (7, 5)];
        const EXPECTED: u32 = 9;
        let calculated = extract_epsilon_rate(INPUT);
        assert_eq!(calculated, EXPECTED);
    }

    #[test]
    fn test_oxygen_generator_rating() {
        let input = vec![4, 30, 22, 23, 21, 15, 7, 28, 16, 25, 2, 10];
        const EXPECTED: u32 = 23;
        let calculated = extract_oxygen_generator_rating(input, 5);
        assert_eq!(calculated, EXPECTED);
    }

    #[test]
    fn test_co2_scrubber_rating() {
        let input = vec![4, 30, 22, 23, 21, 15, 7, 28, 16, 25, 2, 10];
        const EXPECTED: u32 = 10;
        let calculated = extract_co2_scrubber_rating(input, 5);
        assert_eq!(calculated, EXPECTED);
    }
}
