use {
    aoc_2021::*,
    std::{env::args, error::Error, io, io::ErrorKind, time::Instant},
};

fn main() -> Result<(), Box<dyn Error>> {
    if let Some(solution_number) = args().skip(1).next() {
        let start_time = Instant::now();
        run_with(solution_number.parse::<usize>()?, |part_1, part_2| {
            let elapsed = start_time.elapsed().as_micros();
            println!("Part 1 => {}", part_1);
            println!("Part 2 => {}", part_2);
            println!("Took {} microseconds", elapsed);
        });
        Ok(())
    } else {
        Err(Box::new(io::Error::new(
            ErrorKind::InvalidInput,
            "Argument list to program requires an entry indicating the day number to run.",
        )))
    }
}
