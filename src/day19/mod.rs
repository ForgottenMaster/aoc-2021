mod rotations;
mod sensor;

use crate::common::iter::FilterGroupMapExt;
use rotations::*;
use sensor::Sensor;

pub fn run(input: &str) -> (u32, u32) {
    let sensors = parse_into_sensors(input);
    (0, 0)
}

/// Takes the given &str representing the whole input and parses it into
/// the list of sensors, containing the visible points in all the bases.
fn parse_into_sensors(input: &str) -> Vec<Sensor> {
    input
        .trim()
        .lines()
        .filter_group_map(
            |elem| !elem.trim().is_empty(),
            |lines| {
                lines
                    .into_iter()
                    .map(|elem| *elem)
                    .collect::<Vec<_>>()
                    .join("\n")
                    .parse::<Sensor>()
                    .unwrap()
            },
        )
        .collect::<Vec<_>>()
}
