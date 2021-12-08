use std::{fmt::Display, fs::read_to_string};

pub fn run() -> (impl Display, impl Display) {
    let file_content = read_to_string("input/day08.txt").expect("Could not read contents of file.");
    let part_1 = calculate_part_1(&file_content);
    (part_1, 0)
}

fn calculate_part_1(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .split("|")
                .skip(1)
                .next()
                .expect("Could not find separator '|' on one of the input lines.")
                .trim()
                .split(" ")
                .filter(|elem| {
                    let len = elem.trim().len();
                    len == 2 || len == 4 || len == 3 || len == 7
                })
                .count()
        })
        .sum()
}

/// Struct to track the solved wires for a single
/// digit in the display.
#[derive(Default)]
struct SevenSegmentWireSetup {
    top_left: Option<char>,
    top: Option<char>,
    top_right: Option<char>,
    middle: Option<char>,
    bottom_left: Option<char>,
    bottom: Option<char>,
    bottom_right: Option<char>,
}

impl SevenSegmentWireSetup {
    fn segments(&self) -> [&Option<char>; 7] {
        [
            &self.top_left,
            &self.top,
            &self.top_right,
            &self.middle,
            &self.bottom_left,
            &self.bottom,
            &self.bottom_right,
        ]
    }

    fn number_of_segments(&self) -> u8 {
        self.segments()
            .into_iter()
            .filter(|elem| elem.is_some())
            .count() as u8
    }

    fn is_wire_char_in_segments(&self, c: char) -> bool {
        self.segments()
            .into_iter()
            .filter_map(|elem| elem.as_ref())
            .any(|elem| elem == &c)
    }

    fn is_wire_string_matching_segments(&self, s: &str) -> bool {
        let s = s.trim();
        if s.len() as u8 != self.number_of_segments() {
            false // can't match if there's not the correct number of characters
        } else {
            s.chars().all(|c| self.is_wire_char_in_segments(c))
        }
    }
}

/// Struct to track the solved digits of the seven segment.
/// Getting the item at a given index gives you the deduced
/// substring for that character and the deduced wire setup.
#[derive(Default)]
struct SevenSegmentDisplay<'a> {
    digits: [(Option<&'a str>, SevenSegmentWireSetup); 10],
}

impl<'a> SevenSegmentDisplay<'a> {
    fn cache_trivial_strings(&mut self, s: &'a str) {
        for elem in s.trim().split(" ") {
            let elem = elem.trim();
            let len = elem.len();

            if let Some(number) = match len {
                2 => Some(1),
                4 => Some(4),
                3 => Some(7),
                7 => Some(8),
                _ => None,
            } {
                self.digits[number].0 = Some(elem);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seven_segment_wire_segments() {
        const INPUT: SevenSegmentWireSetup = SevenSegmentWireSetup {
            top_left: None,
            top: None,
            top_right: Some('a'),
            middle: None,
            bottom_left: None,
            bottom: None,
            bottom_right: Some('b'),
        };
        const EXPECTED: [&Option<char>; 7] =
            [&None, &None, &Some('a'), &None, &None, &None, &Some('b')];
        let calculated = INPUT.segments();
        assert_eq!(calculated, EXPECTED);
    }

    #[test]
    fn test_seven_segment_wire_setup_number_of_segments() {
        const INPUT: SevenSegmentWireSetup = SevenSegmentWireSetup {
            top_left: None,
            top: None,
            top_right: Some('a'),
            middle: None,
            bottom_left: None,
            bottom: None,
            bottom_right: Some('b'),
        };
        const EXPECTED: u8 = 2;
        let calculated = INPUT.number_of_segments();
        assert_eq!(calculated, EXPECTED);
    }

    #[test]
    fn test_seven_segment_wire_is_char_in_segments_failure() {
        const INPUT: SevenSegmentWireSetup = SevenSegmentWireSetup {
            top_left: None,
            top: None,
            top_right: Some('a'),
            middle: None,
            bottom_left: None,
            bottom: None,
            bottom_right: Some('b'),
        };
        const EXPECTED: bool = false;
        let calculated = INPUT.is_wire_char_in_segments('c');
        assert_eq!(calculated, EXPECTED);
    }

    #[test]
    fn test_seven_segment_wire_is_char_in_segments_success() {
        const INPUT: SevenSegmentWireSetup = SevenSegmentWireSetup {
            top_left: None,
            top: None,
            top_right: Some('a'),
            middle: None,
            bottom_left: None,
            bottom: None,
            bottom_right: Some('b'),
        };
        const EXPECTED: bool = true;
        let calculated = INPUT.is_wire_char_in_segments('b');
        assert_eq!(calculated, EXPECTED);
    }

    #[test]
    fn test_seven_segment_wire_setup_string_too_short() {
        const INPUT: SevenSegmentWireSetup = SevenSegmentWireSetup {
            top_left: None,
            top: None,
            top_right: Some('a'),
            middle: None,
            bottom_left: None,
            bottom: None,
            bottom_right: Some('b'),
        };
        const EXPECTED: bool = false;
        let calculated = INPUT.is_wire_string_matching_segments("a");
        assert_eq!(calculated, EXPECTED);
    }

    #[test]
    fn test_seven_segment_wire_setup_string_too_long() {
        const INPUT: SevenSegmentWireSetup = SevenSegmentWireSetup {
            top_left: None,
            top: None,
            top_right: Some('a'),
            middle: None,
            bottom_left: None,
            bottom: None,
            bottom_right: Some('b'),
        };
        const EXPECTED: bool = false;
        let calculated = INPUT.is_wire_string_matching_segments("abc");
        assert_eq!(calculated, EXPECTED);
    }

    #[test]
    fn test_seven_segment_wire_setup_string_failure() {
        const INPUT: SevenSegmentWireSetup = SevenSegmentWireSetup {
            top_left: None,
            top: None,
            top_right: Some('a'),
            middle: None,
            bottom_left: None,
            bottom: None,
            bottom_right: Some('b'),
        };
        const EXPECTED: bool = false;
        let calculated = INPUT.is_wire_string_matching_segments("ac");
        assert_eq!(calculated, EXPECTED);
    }

    #[test]
    fn test_seven_segment_wire_setup_string_success() {
        const INPUT: SevenSegmentWireSetup = SevenSegmentWireSetup {
            top_left: None,
            top: None,
            top_right: Some('a'),
            middle: None,
            bottom_left: None,
            bottom: None,
            bottom_right: Some('b'),
        };
        const EXPECTED: bool = true;
        let calculated = INPUT.is_wire_string_matching_segments("ab");
        assert_eq!(calculated, EXPECTED);
    }

    #[test]
    fn test_seven_segment_wire_setup_string_success_different_ordering() {
        const INPUT: SevenSegmentWireSetup = SevenSegmentWireSetup {
            top_left: None,
            top: None,
            top_right: Some('a'),
            middle: None,
            bottom_left: None,
            bottom: None,
            bottom_right: Some('b'),
        };
        const EXPECTED: bool = true;
        let calculated = INPUT.is_wire_string_matching_segments("ba");
        assert_eq!(calculated, EXPECTED);
    }

    #[test]
    fn test_seven_segment_display_cache_trivial_strings() {
        let mut input = SevenSegmentDisplay::default();
        input.cache_trivial_strings("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb");
        assert_eq!(input.digits[1].0.unwrap(), "be");
        assert_eq!(input.digits[4].0.unwrap(), "cgeb");
        assert_eq!(input.digits[7].0.unwrap(), "edb");
        assert_eq!(input.digits[8].0.unwrap(), "cfbegad");
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
        const EXPECTED: usize = 26;
        let calculated = calculate_part_1(&INPUT);
        assert_eq!(calculated, EXPECTED);
    }
}
