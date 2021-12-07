mod common;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

use std::{error::Error, fmt::Display, io, io::ErrorKind};

pub fn run(day: usize) -> Result<(Box<dyn Display>, Box<dyn Display>), Box<dyn Error>> {
    let (part_1, part_2) = match day {
        1 => day01::run()?,
        2 => day02::run()?,
        3 => day03::run()?,
        4 => day04::run()?,
        5 => day05::run()?,
        6 => day06::run()?,
        _ => {
            return Err(Box::new(io::Error::new(
                ErrorKind::InvalidInput,
                format!("run called with day {} which is invalid.", day),
            )))
        }
    };
    Ok((part_1, part_2))
}
