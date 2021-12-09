mod number;
mod segment_label;

use {
    number::Number,
    segment_label::SegmentLabel,
    std::{fmt::Display, fs::read_to_string},
};

pub fn run() -> (impl Display, impl Display) {
    let file_content = read_to_string("input/day08.txt").expect("Could not read contents of file.");
    file_content.trim().lines().fold((0, 0), process_line)
}

fn process_line(mut totals: (u32, u32), line: &str) -> (u32, u32) {
    let mut iter = line.trim().split("|");
    let signal_patterns = iter
        .next()
        .expect("A line in the input isn't formatted correctly.")
        .trim();
    let output_numbers = iter
        .next()
        .expect("A line in the input isn't formatted correctly.")
        .trim();

    // Step 1 is to extract the trivial numbers that we can deduce just from the length of the string.
    let number_1: Number = extract_number_with_len(&signal_patterns, 2);
    let number_4: Number = extract_number_with_len(&signal_patterns, 4);
    let number_7: Number = extract_number_with_len(&signal_patterns, 3);
    let number_8: Number = extract_number_with_len(&signal_patterns, 7);

    // We can now isolate the top segment mask because it's present in 7 but not in 1.
    let top_segment: SegmentLabel = (number_7 ^ number_1).into();

    // Number 6 can now be located. 8 xor 7 will give *almost* the pattern for 6, but we need to
    // OR back in the newly determined top_segment.
    let number_6: Number = (number_8 ^ number_7) | top_segment.into();

    totals
}

fn extract_number_with_len(text: &str, len: usize) -> Number {
    text.split(" ")
        .filter(|elem| elem.trim().len() == len)
        .next()
        .unwrap()
        .into()
}
