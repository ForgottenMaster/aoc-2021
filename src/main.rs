use {
    aoc_2021::*,
    std::{env::args, error::Error, time::Instant},
};

/// An enum containing the types of error we could encounter
/// during execution. This would be either an error for having an invalid
/// argument name, or an error thrown from execution of a valid solution.
#[derive(Debug)]
enum ExecutionError {
    InvalidArgument,
    ExecutionError(Box<dyn Error>),
}

impl From<Box<dyn Error>> for ExecutionError {
    fn from(value: Box<dyn Error>) -> Self {
        Self::ExecutionError(value)
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
        let elapsed = start_time.elapsed().as_micros();
        println!("Part 1 => {}", part_1);
        println!("Part 2 => {}", part_2);
        println!("Took {} microseconds", elapsed);
        Ok(())
    } else {
        Err(ExecutionError::InvalidArgument)
    }
}
