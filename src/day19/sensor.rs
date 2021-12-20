use std::{convert::Infallible, str::FromStr};

/// Represents a single Sensor in the puzzle which can detect a certain number of points
/// from whatever its rotation happens to be.
pub struct Sensor {
    points: Vec<(i16, i16, i16)>,
}

impl FromStr for Sensor {
    type Err = Infallible; // just fail on error.

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let string = string.trim();
        dbg!(string);
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {}
