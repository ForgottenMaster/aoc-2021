use {
    aoc_2021::*,
    std::{env::args, time::Instant},
};

/// An enum containing the types of error we could encounter
/// during execution. This would be either an error for having an invalid
/// argument name, or an error thrown from execution of a valid solution.
#[derive(Debug)]
enum ExecutionError {
    InvalidArgument,
    Day01Error(day01::ExecutionError),
    Day02Error(day02::ExecutionError),
    Day03Error(day03::ExecutionError),
}

impl From<day01::ExecutionError> for ExecutionError {
    fn from(value: day01::ExecutionError) -> Self {
        Self::Day01Error(value)
    }
}

impl From<day02::ExecutionError> for ExecutionError {
    fn from(value: day02::ExecutionError) -> Self {
        Self::Day02Error(value)
    }
}

impl From<day03::ExecutionError> for ExecutionError {
    fn from(value: day03::ExecutionError) -> Self {
        Self::Day03Error(value)
    }
}

fn main() -> Result<(), ExecutionError> {
    if let Some(solution_number) = args().skip(1).next() {
        let start_time = Instant::now();
        let (part_1, part_2) = match solution_number.trim() {
            "1" => day01::run()?,
            "2" => day02::run()?,
            "3" => day03::run()?,
            _ => return Err(ExecutionError::InvalidArgument),
        };
        println!("Part 1 => {}", part_1);
        println!("Part 2 => {}", part_2);
        println!("Took {} microseconds", start_time.elapsed().as_micros());
        Ok(())
    } else {
        Err(ExecutionError::InvalidArgument)
    }
}
