use std::{
    collections::HashMap,
    fmt::Display,
    io::{BufRead, BufReader},
};

pub fn run(input: &str) -> (impl Display, impl Display) {
    let input = input.as_bytes();
    let reader = BufReader::new(input);
    let (crab_position_counts, min, max) = read_crab_position_counts(reader);
    let part_1 = calculate_minimal_fuel_cost_to_align(&crab_position_counts, min, max, 0);
    let part_2 = calculate_minimal_fuel_cost_to_align(&crab_position_counts, min, max, 1);
    (part_1, part_2)
}

/// Takes a given BufReader that should contain a sequence of comma separated
/// unsigned integers and returns a HashMap containing the positions mapped to the
/// number that are at that position (this is because we will always move all crabs at a given
/// position rather than individual crabs).
/// Along with the hashmap of positions returns the minimum and maximum position that has a crab on it.
fn read_crab_position_counts(reader: impl BufRead) -> (HashMap<u32, u32>, u32, u32) {
    let mut hm = HashMap::new();
    let mut min = u32::MAX;
    let mut max = u32::MIN;
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
                    min = std::cmp::min(min, elem);
                    max = std::cmp::max(max, elem);
                });
        });
    (hm, min, max)
}

/// Takes a reference to a mapping of crab position counts along with a given position
/// and calculates the fuel required to align all crabs on that position.
fn calculate_fuel_required_for_position(
    position_counts: &HashMap<u32, u32>,
    target_position: u32,
    fuel_acceleration: u32,
    maximum_allowed_fuel: u32,
) -> u32 {
    let mut total_fuel = 0;
    position_counts.into_iter().for_each(|(position, count)| {
        let (min, max) = if *position > target_position {
            (target_position, *position)
        } else {
            (*position, target_position)
        };
        let diff = max - min;
        let allowed_fuel_overestimate =
            maximum_allowed_fuel - std::cmp::min(total_fuel, maximum_allowed_fuel);
        total_fuel +=
            calculate_fuel_cost_from_distance(diff, fuel_acceleration, allowed_fuel_overestimate)
                * count
        // required as we're shifting all crabs at that position
    });
    total_fuel
}

fn calculate_fuel_cost_from_distance(
    steps: u32,
    fuel_acceleration: u32,
    maximum_allowed_fuel: u32,
) -> u32 {
    let mut i = 0;
    let mut total_fuel = 0;
    let mut fuel_cost_per_unit = 1;

    while total_fuel < maximum_allowed_fuel && i < steps {
        total_fuel += fuel_cost_per_unit;
        fuel_cost_per_unit += fuel_acceleration;
        i += 1;
    }

    std::cmp::min(total_fuel, maximum_allowed_fuel)
}

/// Calculates the minimal fuel cost for arranging the crabs.
fn calculate_minimal_fuel_cost_to_align(
    position_counts: &HashMap<u32, u32>,
    min_pos: u32,
    max_pos: u32,
    fuel_acceleration: u32,
) -> u32 {
    let mut minimal_fuel = u32::MAX;
    (min_pos..=max_pos).into_iter().for_each(|position| {
        minimal_fuel = std::cmp::min(
            minimal_fuel,
            calculate_fuel_required_for_position(
                position_counts,
                position,
                fuel_acceleration,
                minimal_fuel,
            ),
        );
    });
    minimal_fuel
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
        let (calculated, _, _) = read_crab_position_counts(INPUT);
        assert_eq!(expected, calculated);
    }

    #[test]
    fn test_calculate_fuel_required_for_position_constant_acceleration() {
        let input_counts = [(0, 1), (1, 2), (2, 3), (4, 1), (7, 1), (14, 1), (16, 1)]
            .into_iter()
            .collect::<HashMap<_, _>>();
        let input_position = 2;
        let expected = 37;
        let calculated =
            calculate_fuel_required_for_position(&input_counts, input_position, 0, u32::MAX);
        assert_eq!(expected, calculated);
    }

    #[test]
    fn test_calculate_fuel_required_for_position_ramping_acceleration() {
        let input_counts = [(0, 1), (1, 2), (2, 3), (4, 1), (7, 1), (14, 1), (16, 1)]
            .into_iter()
            .collect::<HashMap<_, _>>();
        let input_position = 2;
        let expected = 206;
        let calculated =
            calculate_fuel_required_for_position(&input_counts, input_position, 1, u32::MAX);
        assert_eq!(expected, calculated);
    }

    #[test]
    fn test_calculate_minimal_fuel_cost_to_align_constant_acceleration() {
        let input_counts = [(0, 1), (1, 2), (2, 3), (4, 1), (7, 1), (14, 1), (16, 1)]
            .into_iter()
            .collect::<HashMap<_, _>>();
        let expected = 37;
        let calculated = calculate_minimal_fuel_cost_to_align(&input_counts, 0, 16, 0);
        assert_eq!(expected, calculated);
    }

    #[test]
    fn test_calculate_minimal_fuel_cost_to_align_ramping_acceleration() {
        let input_counts = [(0, 1), (1, 2), (2, 3), (4, 1), (7, 1), (14, 1), (16, 1)]
            .into_iter()
            .collect::<HashMap<_, _>>();
        let expected = 168;
        let calculated = calculate_minimal_fuel_cost_to_align(&input_counts, 0, 16, 1);
        assert_eq!(expected, calculated);
    }

    #[test]
    fn test_calculate_fuel_cost_from_distance_ramping() {
        assert_eq!(calculate_fuel_cost_from_distance(3, 0, u32::MAX), 3);
        assert_eq!(calculate_fuel_cost_from_distance(3, 1, u32::MAX), 6);
        assert_eq!(calculate_fuel_cost_from_distance(10, 0, u32::MAX), 10);
        assert_eq!(calculate_fuel_cost_from_distance(10, 1, u32::MAX), 55);
        assert_eq!(calculate_fuel_cost_from_distance(11, 1, u32::MAX), 66);
        assert_eq!(calculate_fuel_cost_from_distance(4, 1, u32::MAX), 10);
        assert_eq!(calculate_fuel_cost_from_distance(5, 1, u32::MAX), 15);
        assert_eq!(calculate_fuel_cost_from_distance(1, 1, u32::MAX), 1);
        assert_eq!(calculate_fuel_cost_from_distance(2, 1, u32::MAX), 3);
        assert_eq!(calculate_fuel_cost_from_distance(9, 1, u32::MAX), 45);
        assert_eq!(calculate_fuel_cost_from_distance(0, 1, u32::MAX), 0);
    }

    #[test]
    fn test_calculate_fuel_required_for_position_4() {
        let input_counts = [(0, 1), (1, 2), (2, 3), (4, 1), (7, 1), (14, 1), (16, 1)]
            .into_iter()
            .collect::<HashMap<_, _>>();
        let input_position = 5;
        let expected = 168;
        let calculated =
            calculate_fuel_required_for_position(&input_counts, input_position, 1, u32::MAX);
        assert_eq!(expected, calculated);
    }
}
