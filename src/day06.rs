use std::{
    collections::HashMap,
    fmt::Display,
    io::{BufRead, BufReader},
    num::ParseIntError,
};

const RESET_TO: u8 = 6;
const SPAWN_AT: u8 = 8;

pub fn run(input: &str) -> (impl Display, impl Display) {
    let input = input.as_bytes();
    let reader = BufReader::new(input);
    let mut fish_counts =
        read_lanternfish_counts(reader).expect("Could not read lanternfish counts from file.");
    simulate_days(&mut fish_counts, 80);
    let part_1: u128 = fish_counts.iter().map(|(_, value)| value).sum();
    simulate_days(&mut fish_counts, 176); // part 2 is 176 iterations more than part 1
    let part_2: u128 = fish_counts.iter().map(|(_, value)| value).sum();
    (part_1, part_2)
}

fn read_lanternfish_counts(reader: impl BufRead) -> Result<HashMap<u8, u128>, ParseIntError> {
    let mut fish_counts = HashMap::new();
    for line in reader.lines().filter_map(|line| Some(line.ok()?)) {
        let line = line.trim();
        if !line.is_empty() {
            for elem in line.split(",") {
                *fish_counts.entry(elem.trim().parse::<u8>()?).or_insert(0) += 1;
            }
        }
    }
    Ok(fish_counts)
}

fn simulate_one_day(fish_counts: &mut HashMap<u8, u128>) {
    // cache the number of fish currently at 0 (that will be added on at the end, and will spawn new fish).
    let number_of_zeroes = {
        let entry = fish_counts.entry(0).or_insert(0);
        let number_of_zeroes = *entry;
        *entry = 0;
        number_of_zeroes
    };

    (1..=SPAWN_AT).for_each(|idx| {
        let entry = fish_counts.entry(idx).or_insert(0);
        let count = *entry;
        *entry = 0;
        let entry = fish_counts.entry(idx - 1).or_insert(0);
        *entry += count;
    });

    // add both the fish that are cycling and their spawned offspring.
    *fish_counts.entry(RESET_TO).or_insert(0) += number_of_zeroes;
    *fish_counts.entry(SPAWN_AT).or_insert(0) += number_of_zeroes;
}

fn simulate_days(fish_counts: &mut HashMap<u8, u128>, days: u128) {
    (0..days).for_each(|_| simulate_one_day(fish_counts));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulate_1_day() {
        const INPUT: &[u8] = "3, 4, 3, 1, 2".as_bytes();
        const EXPECTED: &[u8] = "2, 3, 2, 0, 1".as_bytes();
        let mut fish_counts = read_lanternfish_counts(INPUT).unwrap();
        let expected_counts = read_lanternfish_counts(EXPECTED).unwrap();
        simulate_days(&mut fish_counts, 1);
        assert_eq!(
            fish_counts.iter().map(|(_, value)| value).sum::<u128>(),
            expected_counts.iter().map(|(_, value)| value).sum()
        );
    }

    #[test]
    fn test_simulate_5_days() {
        const INPUT: &[u8] = "3, 4, 3, 1, 2".as_bytes();
        const EXPECTED: &[u8] = "5, 6, 5, 3, 4, 5, 6, 7, 7, 8".as_bytes();
        let mut fish_counts = read_lanternfish_counts(INPUT).unwrap();
        let expected_counts = read_lanternfish_counts(EXPECTED).unwrap();
        simulate_days(&mut fish_counts, 5);
        assert_eq!(
            fish_counts.iter().map(|(_, value)| value).sum::<u128>(),
            expected_counts.iter().map(|(_, value)| value).sum()
        );
    }

    #[test]
    fn test_example_part_1() {
        const INPUT: &[u8] = "3,4,3,1,2".as_bytes();
        const EXPECTED: u128 = 5934;
        let mut fish_counts = read_lanternfish_counts(INPUT).unwrap();
        simulate_days(&mut fish_counts, 80);
        assert_eq!(
            fish_counts.iter().map(|(_, value)| value).sum::<u128>(),
            EXPECTED
        );
    }

    #[test]
    fn test_example_part_2() {
        const INPUT: &[u8] = "3,4,3,1,2".as_bytes();
        const EXPECTED: u128 = 26984457539;
        let mut fish_counts = read_lanternfish_counts(INPUT).unwrap();
        simulate_days(&mut fish_counts, 256);
        assert_eq!(
            fish_counts.iter().map(|(_, value)| value).sum::<u128>(),
            EXPECTED
        );
    }
}
