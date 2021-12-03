use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

fn main() -> std::io::Result<()> {
    let start_time = Instant::now();
    let file = File::open("input/day03.txt")?;
    let reader = BufReader::new(file);
    let part_1 = run(reader);
    println!("Part 1 => {}", part_1);
    let end_time = Instant::now();
    let duration = end_time.duration_since(start_time);
    println!("Took {} nanoseconds to run", duration.as_nanos());
    Ok(())
}

/// Calculates the answer to the problem in a single pass over the buffer.
/// Ignores any lines that are empty, don't have the same length as the preceding lines, or
/// have an invalid character in it (should just be leading/trailing whitespace and then binary only).
fn run(reader: impl BufRead) -> u32 {
    let (gamma_rate, epsilon_rate) = convert_rate_streams_to_decimal(generate_rate_streams(
        digit_counts(valid_line_iter(reader)),
    ));
    gamma_rate * epsilon_rate
}

/// Converts the stream of 0's and 1's into the decimal equivalent and
/// returns them as the final answers.
/// The first element of the output tuple is the gamma rate, and the second is the
/// epsilon rate.
fn convert_rate_streams_to_decimal(iter: impl Iterator<Item = (u32, u32)>) -> (u32, u32) {
    iter.enumerate().fold(
        (0, 0),
        |(result_gamma, result_epsilon), (idx, (elem_gamma, elem_epsilon))| {
            if idx == 0 {
                (elem_gamma, elem_epsilon)
            } else {
                (
                    (result_gamma << 1) + elem_gamma,
                    (result_epsilon << 1) + elem_epsilon,
                )
            }
        },
    )
}

/// Takes an iterator over column counts of 0's and 1's and produces an iterator over
/// pairs of the bits of the two "rates". The first entry in each output tuple is the
/// bits of the gamma rate, and the second are the bits of the epsilon rate.
fn generate_rate_streams(
    iter: impl Iterator<Item = (u32, u32)>,
) -> impl Iterator<Item = (u32, u32)> {
    iter.map(|(count_0, count_1)| (if count_0 > count_1 { (0, 1) } else { (1, 0) }))
}

/// Converts a stream of valid lines (note the type system doesn't help with that, if we wanted
/// to be more rigorous we can use a type that's only constructible with a valid line to be sure).
/// Folds the lines together resulting in a stream over tuples (same length as a line) where the first
/// element of the tuple is the number of 0's in that column, and the second is the number of 1's.
fn digit_counts(iter: impl Iterator<Item = String>) -> impl Iterator<Item = (u32, u32)> {
    iter.fold(Vec::<(u32, u32)>::new(), |mut state, elem| {
        let iter = elem
            .trim()
            .chars()
            .map(|c| if c == '1' { (0, 1) } else { (1, 0) });

        // either update or create the vector
        if state.len() > 0 {
            iter.enumerate().for_each(|(idx, (delta_0, delta_1))| {
                let (count_0, count_1) = state[idx];
                state[idx] = (count_0 + delta_0, count_1 + delta_1);
            });
            state
        } else {
            iter.collect()
        }
    })
    .into_iter()
}

/// Handles wrapping the reader up into an iterator that will remove any
/// invalid lines as it's iterating and ensure all the lines are the same
/// length. Resulting iterator is guaranteed to only iterate over lines that are:
/// 1) Non empty
/// 2) Same length (as the first valid line)
/// 3) Consist only of 1's and 0's (not including whitespace)
fn valid_line_iter(reader: impl BufRead) -> impl Iterator<Item = String> {
    reader
        .lines()
        .filter_map(|elem| {
            let mut len = 0;
            let mut invalid_char_count = 0;
            let elem = elem.ok()?;
            elem.trim().chars().for_each(|c| {
                len += 1;
                if c != '0' && c != '1' {
                    invalid_char_count += 1;
                }
            });

            if len == 0 || invalid_char_count > 0 {
                None
            } else {
                Some((len, elem))
            }
        })
        .scan(0, |line_len, (elem_len, elem)| {
            if *line_len == 0 {
                *line_len = elem_len;
                Some(elem)
            } else if elem_len != *line_len {
                None
            } else {
                Some(elem)
            }
        })
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn test_example_part_1() {
        const INPUT: &[u8] = r#"
        00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010
        "#
        .as_bytes();
        const EXPECTED: u32 = 198;
        let calculated = run(INPUT);
        assert_eq!(calculated, EXPECTED);
    }
}
