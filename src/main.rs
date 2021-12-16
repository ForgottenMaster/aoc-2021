use {
    aoc_2021::*,
    std::{env::args, time::Instant},
};

fn main() {
    if let Some(solution_number) = args().skip(1).next() {
        let solution_number = solution_number
            .parse::<usize>()
            .expect("Failed to parse provided command line argument as a number.");
        let start_time = Instant::now();
        run_with(solution_number, |part_1, part_2| {
            let elapsed = start_time.elapsed().as_micros();
            println!("Part 1 => {}", part_1);
            println!("Part 2 => {}", part_2);
            println!("Took {} microseconds", elapsed);
        });
    } else {
        panic!("Argument not provided, should be run with the day number.");
    }
}
