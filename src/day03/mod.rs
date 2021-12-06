mod diagnostic_report;
mod read_diagnostic_report_error;

use {
    diagnostic_report::DiagnosticReport,
    std::{error, fs::File, io::BufReader, iter::repeat},
};

pub fn run() -> Result<(Box<u32>, Box<u32>), Box<dyn error::Error>> {
    let file = File::open("input/day03.txt")?;
    let reader = BufReader::new(file);
    let diagnostic_report = DiagnosticReport::<u32>::new_from_bufread(reader)?.unwrap();
    let bit_count = if diagnostic_report.len() > 0 {
        diagnostic_report[0].input_string_bit_count()
    } else {
        0
    };
    let diagnostic_report = diagnostic_report
        .into_iter()
        .map(|elem| *elem.parsed())
        .collect::<Vec<_>>();

    let bit_counts = calculate_bit_counts(&diagnostic_report, bit_count);
    let gamma_rate = extract_gamma_rate(&bit_counts);
    let epsilon_rate = extract_epsilon_rate(&bit_counts);
    let part_1 = gamma_rate * epsilon_rate;

    let oxygen_generator_rating =
        extract_oxygen_generator_rating(diagnostic_report.clone(), bit_count);
    let co2_scrubber_rating = extract_co2_scrubber_rating(diagnostic_report, bit_count);
    let part_2 = oxygen_generator_rating * co2_scrubber_rating;

    Ok((Box::new(part_1), Box::new(part_2)))
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

#[cfg(test)]
mod tests {
    use super::*;

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
