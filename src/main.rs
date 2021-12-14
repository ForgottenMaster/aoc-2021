use {
    aoc_2021::*,
    std::{env::args, num::ParseIntError, time::Instant},
};

/// Used to create a homogenous return type for the program execution rather than invoking a panic.
/// This allows us to propagate correctly up the stack and out of the main function and allows for
/// us to ensure code coverage.
#[derive(Debug, PartialEq)]
enum ProgramError {
    ParseIntError(ParseIntError), // error from parsing the solution number to run
    InvalidArgument,              // error caused by no argument being found at the command line
    ExecutionError(ExecutionError), // error propagated from the execution of the solution
}

impl From<ParseIntError> for ProgramError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

impl From<ExecutionError> for ProgramError {
    fn from(value: ExecutionError) -> Self {
        Self::ExecutionError(value)
    }
}

/// Function that actually does the parsing of the command line argument to determine which
/// day to run, taking only an iterator of strings which enables us to run this in both
/// a test context and a runtime context.
fn main_internal(args: impl Iterator<Item = String>) -> Result<(), ProgramError> {
    if let Some(solution_number) = args.skip(1).next() {
        let solution_number = solution_number.parse::<usize>()?;
        let start_time = Instant::now();
        run_with(solution_number, |part_1, part_2| {
            let elapsed = start_time.elapsed().as_micros();
            println!("Part 1 => {}", part_1);
            println!("Part 2 => {}", part_2);
            println!("Took {} microseconds", elapsed);
        })?;
        Ok(())
    } else {
        Err(ProgramError::InvalidArgument)
    }
}

fn main() -> Result<(), ProgramError> {
    main_internal(args())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test here just to allow code coverage to be reported as 100% by covering the main function itself.
    // We can't mock the environment arguments though so we expect this to return an error.
    #[test]
    fn test_main() {
        assert!(main().is_err());
    }

    #[test]
    fn test_main_internal_invalid_argument() {
        let calculated = main_internal(vec!["program_path".to_string()].into_iter()).unwrap_err();
        let expected = ProgramError::InvalidArgument;
        assert_eq!(calculated, expected);
    }

    #[test]
    fn test_main_internal_parse_int_error() {
        let calculated =
            main_internal(vec!["program_path".to_string(), "foo".to_string()].into_iter())
                .unwrap_err();
        let matches = matches!(calculated, ProgramError::ParseIntError(..));
        assert!(matches);
    }

    #[test]
    fn test_main_internal_execution_error() {
        let calculated =
            main_internal(vec!["program_path".to_string(), "26".to_string()].into_iter())
                .unwrap_err();
        let expected = ProgramError::ExecutionError(ExecutionError::InvalidDay(26));
        assert_eq!(calculated, expected);
    }

    #[test]
    fn test_main_internal_success() {
        let calculated =
            main_internal(vec!["program_path".to_string(), "1".to_string()].into_iter()).is_ok();
        assert!(calculated);
    }

    #[test]
    fn test_program_error_from_parse_int_error() {
        let parsed = "foo".parse::<u32>().unwrap_err();
        let converted: ProgramError = parsed.clone().into();
        let expected = ProgramError::ParseIntError(parsed);
        assert_eq!(converted, expected);
    }

    #[test]
    fn test_program_error_from_execution_error() {
        let converted: ProgramError = ExecutionError::InvalidDay(26).into();
        let expected = ProgramError::ExecutionError(ExecutionError::InvalidDay(26));
        assert_eq!(converted, expected);
    }
}
