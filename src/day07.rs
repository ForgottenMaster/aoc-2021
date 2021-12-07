use std::{
    collections::HashMap,
    error::Error,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn run() -> Result<(Box<dyn Display>, Box<dyn Display>), Box<dyn Error>> {
    let file = File::open("input/day07.txt")?;
    let reader = BufReader::new(file);
    let crab_position_counts = read_crab_position_counts(reader);
    let part_1 = calculate_minimal_fuel_cost_to_align(&crab_position_counts);
    Ok((Box::new(part_1), Box::new(0)))
}

/// Takes a given BufReader that should contain a sequence of comma separated
/// unsigned integers and returns a HashMap containing the positions mapped to the
/// number that are at that position (this is because we will always move all crabs at a given
/// position rather than individual crabs).
fn read_crab_position_counts(reader: impl BufRead) -> HashMap<u32, u32> {
    let mut hm = HashMap::new();
    reader
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| {
            line.trim()
                .split(",")
                .filter_map(|elem| {
                    let elem = elem.trim();
                    if elem.is_empty() {
                        None
                    } else {
                        Some(elem.parse().unwrap())
                    }
                })
                .for_each(|elem: u32| {
                    *hm.entry(elem).or_insert(0) += 1;
                });
        });
    hm
}

/// Takes a reference to a mapping of crab position counts along with a given position
/// and calculates the fuel required to align all crabs on that position.
fn calculate_fuel_required_for_position(
    position_counts: &HashMap<u32, u32>,
    target_position: u32,
) -> u32 {
    position_counts
        .into_iter()
        .map(|(position, count)| {
            let (min, max) = if *position > target_position {
                (target_position, *position)
            } else {
                (*position, target_position)
            };
            let diff = max - min;
            diff * count // required as we're shifting all crabs at that position
        })
        .sum()
}

/// Calculates the minimal fuel cost for arranging the crabs.
fn calculate_minimal_fuel_cost_to_align(position_counts: &HashMap<u32, u32>) -> u32 {
    position_counts
        .iter()
        .map(|(position, _)| calculate_fuel_required_for_position(position_counts, *position))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_crab_positions() {
        const INPUT: &[u8] = r#"
        16,1,2,0,4,2,7,1,2,14
        "#
        .as_bytes();
        let expected = [(0, 1), (1, 2), (2, 3), (4, 1), (7, 1), (14, 1), (16, 1)]
            .into_iter()
            .collect::<HashMap<_, _>>();
        let calculated = read_crab_position_counts(INPUT);
        assert_eq!(expected, calculated);
    }

    #[test]
    fn test_calculate_fuel_required_for_position() {
        let input_counts = [(0, 1), (1, 2), (2, 3), (4, 1), (7, 1), (14, 1), (16, 1)]
            .into_iter()
            .collect::<HashMap<_, _>>();
        let input_position = 2;
        let expected = 37;
        let calculated = calculate_fuel_required_for_position(&input_counts, input_position);
        assert_eq!(expected, calculated);
    }

    #[test]
    fn test_calculate_minimal_fuel_cost_to_align() {
        let input_counts = [(0, 1), (1, 2), (2, 3), (4, 1), (7, 1), (14, 1), (16, 1)]
            .into_iter()
            .collect::<HashMap<_, _>>();
        let expected = 37;
        let calculated = calculate_minimal_fuel_cost_to_align(&input_counts);
        assert_eq!(expected, calculated);
    }
}
