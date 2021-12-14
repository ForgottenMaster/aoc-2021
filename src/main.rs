use {
    aoc_2021::*,
    std::{env::args, time::Instant},
};

/// Function that actually does the parsing of the command line argument to determine which
/// day to run, taking only an iterator of strings which enables us to run this in both
/// a test context and a runtime context.
fn main_internal(args: impl Iterator<Item = String>) {
    if let Some(solution_number) = args.skip(1).next() {
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

fn main() {
    main_internal(args())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test here just to allow code coverage to be reported as 100% by covering the main function itself.
    // We can't mock the environment arguments though so we expect this to panic.
    #[test]
    #[should_panic]
    fn test_main() {
        main();
    }

    #[test]
    #[should_panic]
    fn test_main_internal_invalid_argument() {
        main_internal(vec!["program_path".to_string()].into_iter());
    }

    #[test]
    #[should_panic]
    fn test_main_internal_parse_int_error() {
        main_internal(vec!["program_path".to_string(), "foo".to_string()].into_iter());
    }

    #[test]
    #[should_panic]
    fn test_main_internal_execution_error() {
        main_internal(vec!["program_path".to_string(), "26".to_string()].into_iter());
    }

    #[test]
    fn test_main_internal_success() {
        main_internal(vec!["program_path".to_string(), "1".to_string()].into_iter());
    }
}
