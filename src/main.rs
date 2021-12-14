use {
    aoc_2021::*,
    std::{env::args, time::Instant},
};

fn main_internal(args: impl Iterator<Item = String>) {
    if let Some(solution_number) = args.skip(1).next() {
        let start_time = Instant::now();
        run_with(
            solution_number.parse::<usize>().unwrap(),
            |part_1, part_2| {
                let elapsed = start_time.elapsed().as_micros();
                println!("Part 1 => {}", part_1);
                println!("Part 2 => {}", part_2);
                println!("Took {} microseconds", elapsed);
            },
        );
    } else {
        panic!("Argument list to program requires an entry indicating the day number to run.");
    }
}

fn main() {
    main_internal(args());
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test here just to allow code coverage to be reported as 100%
    #[test]
    #[should_panic]
    fn test_main() {
        main();
    }

    #[test]
    #[should_panic]
    fn test_main_internal_invalid_argument() {
        main_internal(vec!["program_path".to_string(), "foo".to_string()].into_iter());
    }

    #[test]
    fn test_main_internal_valid_argument() {
        main_internal(vec!["program_path".to_string(), "1".to_string()].into_iter());
    }
}
