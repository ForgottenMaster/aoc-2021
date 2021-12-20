mod rotations;
mod sensor;

use rotations::*;
use sensor::Sensor;

pub fn run(input: &str) -> (u32, u32) {
    let sensors = parse_into_sensors(input);
    (0, 0)
}

/// Takes the given &str representing the whole input and parses it into
/// the list of sensors, containing the visible points in all the bases.
fn parse_into_sensors(input: &str) -> Vec<Sensor> {
    unimplemented!()
}
