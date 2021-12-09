mod segment_label;

use {
    segment_label::SegmentLabel,
    std::{fmt::Display, fs::read_to_string},
};

pub fn run() -> (impl Display, impl Display) {
    let file_content = read_to_string("input/day08.txt").expect("Could not read contents of file.");
    process_content(&file_content)
}

fn process_content(string: &str) -> (u32, u32) {
    string.trim().lines().fold((0, 0), process_line)
}

fn process_line(totals: (u32, u32), line: &str) -> (u32, u32) {
    let mut iter = line.trim().split("|");
    let signal_patterns = iter
        .next()
        .expect("A line in the input isn't formatted correctly.")
        .trim();
    let output_numbers = iter
        .next()
        .expect("A line in the input isn't formatted correctly.")
        .trim();
    let codex = decode_line(signal_patterns);
    let (part_1_digit_count, final_number) =
        output_numbers.split(" ").fold((0, 0), |mut state, digit| {
            let digit = decode_digit(digit, &codex);
            if match digit {
                1 | 4 | 7 | 8 => true,
                _ => false,
            } {
                state.0 += 1;
            }
            state.1 = state.1 * 10 + digit as u32;
            state
        });
    (totals.0 + part_1_digit_count, totals.1 + final_number)
}

/// Decodes a given string as a single digit using the decoded numbers provided. Essentially just a bitmask
/// check.
fn decode_digit(number: &str, codex: &[u8]) -> u8 {
    let number = parse_bitmask_from_string(number);
    codex
        .into_iter()
        .enumerate()
        .filter_map(|(idx, elem)| {
            if *elem == number {
                Some(idx as u8)
            } else {
                None
            }
        })
        .next()
        .unwrap()
}

/// Takes a single line of signal patterns and decodes them, returning the bit patterns of the numbers
/// 0 through 9 in an array.
fn decode_line(line: &str) -> [u8; 10] {
    let line = line.trim();

    // Firstly we need to extract the trivial signal patterns that we know correspond to
    // numbers 1, 4, 7, and 8 due to their character lengths. Once we have these we can start
    // deducing the other numbers using bitwise operations.
    let number_1 = extract_bitmask_with_len(line, 2, |_| true);
    let number_4 = extract_bitmask_with_len(line, 4, |_| true);
    let number_7 = extract_bitmask_with_len(line, 3, |_| true);
    let number_8 = extract_bitmask_with_len(line, 7, |_| true);
    let number_3 = extract_bitmask_with_len(line, 5, |elem| elem & number_1 == number_1); // filters out the other 5 segments digits (5 and 2) because neither has both active segments as number 1.
    let number_6 = extract_bitmask_with_len(line, 6, |elem| elem & number_1 != number_1);
    let number_9 = extract_bitmask_with_len(line, 6, |elem| elem & number_3 == number_3);
    let number_0 = extract_bitmask_with_len(line, 6, |elem| elem != number_6 && elem != number_9);
    let number_2 = extract_bitmask_with_len(line, 5, |elem| ((elem & number_1) & number_6) == 0);
    let number_5 = extract_bitmask_with_len(line, 5, |elem| elem != number_3 && elem != number_2);

    // Now we have all of the numbers sorted out we can order them in an array for parsing the output numbers.
    [
        number_0, number_1, number_2, number_3, number_4, number_5, number_6, number_7, number_8,
        number_9,
    ]
}

/// Takes a string containing multiple signal patterns using labels for the segments
/// such as d, c, a, etc. and finds one bitmask with a specific length (using a specific number
/// of segments). It will then parse this bitmask as the appropriate u8 for later use.
/// Panics if there is no such length in the string as it indicates an invalid format.
fn extract_bitmask_with_len(text: &str, len: usize, predicate: impl Fn(u8) -> bool) -> u8 {
    text.split(" ")
        .filter_map(|elem| {
            let elem = elem.trim();
            if elem.len() == len {
                let elem = parse_bitmask_from_string(elem);
                if predicate(elem) {
                    Some(elem)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .next()
        .unwrap()
}

/// Takes a string representing a valid 7-segment display number and converts it to
/// the bitmask for the segments included.
/// e.g. if the input string is "acd" then the output will be:
/// SegmentLabel.A | SegmentLabel.C | SegmentLabel.D
/// Note: panics if the input string contains invalid character to form the bitmask.
fn parse_bitmask_from_string(string: &str) -> u8 {
    string
        .trim()
        .chars()
        .map(|c| SegmentLabel::from(c) as u8)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_bitmask_with_len_valid() {
        const INPUT: &str = "def afdc ab";
        const EXPECTED: u8 = SegmentLabel::A as u8
            | SegmentLabel::F as u8
            | SegmentLabel::D as u8
            | SegmentLabel::C as u8;
        let calculated = extract_bitmask_with_len(INPUT, 4, |elem| true);
        assert_eq!(calculated, EXPECTED);
    }

    #[test]
    #[should_panic]
    fn test_extract_bitmask_with_len_invalid() {
        extract_bitmask_with_len("def afdc ab", 5, |elem| true);
    }

    #[test]
    fn test_parse_bitmask_from_string_valid() {
        const INPUT: &str = "afdc";
        const EXPECTED: u8 = SegmentLabel::A as u8
            | SegmentLabel::F as u8
            | SegmentLabel::D as u8
            | SegmentLabel::C as u8;
        let calculated = parse_bitmask_from_string(INPUT);
        assert_eq!(calculated, EXPECTED);
    }

    #[test]
    #[should_panic]
    fn test_parse_bitmask_from_string_invalid() {
        parse_bitmask_from_string("dezf");
    }

    #[test]
    fn test_decode_line() {
        const INPUT: &str = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab";
        let expected = [
            parse_bitmask_from_string("cagedb"),
            parse_bitmask_from_string("ab"),
            parse_bitmask_from_string("gcdfa"),
            parse_bitmask_from_string("fbcad"),
            parse_bitmask_from_string("eafb"),
            parse_bitmask_from_string("cdfbe"),
            parse_bitmask_from_string("cdfbeg"),
            parse_bitmask_from_string("dab"),
            parse_bitmask_from_string("acedgfb"),
            parse_bitmask_from_string("cefabd"),
        ];
        assert_eq!(decode_line(INPUT), expected);
    }

    #[test]
    fn test_decode_digit() {
        let codex = [
            parse_bitmask_from_string("cagedb"),
            parse_bitmask_from_string("ab"),
            parse_bitmask_from_string("gcdfa"),
            parse_bitmask_from_string("fbcad"),
            parse_bitmask_from_string("eafb"),
            parse_bitmask_from_string("cdfbe"),
            parse_bitmask_from_string("cdfbeg"),
            parse_bitmask_from_string("dab"),
            parse_bitmask_from_string("acedgfb"),
            parse_bitmask_from_string("cefabd"),
        ];
        const INPUT_1: &str = "cdfeb";
        const INPUT_2: &str = "fcadb";
        const INPUT_3: &str = "cdbaf";
        const EXPECTED_1: u8 = 5;
        const EXPECTED_2: u8 = 3;
        const EXPECTED_3: u8 = 3;
        assert_eq!(decode_digit(&INPUT_1, &codex), EXPECTED_1);
        assert_eq!(decode_digit(&INPUT_2, &codex), EXPECTED_2);
        assert_eq!(decode_digit(&INPUT_3, &codex), EXPECTED_3);
    }

    #[test]
    fn test_part_1_example() {
        const INPUT: &str = r#"
        be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
        "#;
        const EXPECTED: u32 = 26;
        assert_eq!(process_content(INPUT).0, EXPECTED);
    }

    #[test]
    fn test_part_2_example() {
        const INPUT: &str = r#"
        be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
        "#;
        const EXPECTED: u32 = 61229;
        assert_eq!(process_content(INPUT).1, EXPECTED);
    }
}
