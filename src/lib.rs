mod common;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

use std::fmt::Display;

pub fn run_with(day: usize, func: impl Fn(&dyn Display, &dyn Display)) {
    match day {
        1 => call_with(day01::run, func),
        2 => call_with(day02::run, func),
        3 => call_with(day03::run, func),
        4 => call_with(day04::run, func),
        5 => call_with(day05::run, func),
        6 => call_with(day06::run, func),
        7 => call_with(day07::run, func),
        8 => call_with(day08::run, func),
        9 => call_with(day09::run, func),
        10 => call_with(day10::run, func),
        11 => call_with(day11::run, func),
        12 => call_with(day12::run, func),
        13 => call_with(day13::run, func),
        14 => call_with(day14::run, func),
        15 => call_with(day15::run, func),
        16 => call_with(day16::run, func),
        17 => call_with(day17::run, func),
        18 => call_with(day18::run, func),
        19 => call_with(day19::run, func),
        20 => call_with(day20::run, func),
        21 => call_with(day21::run, func),
        22 => call_with(day22::run, func),
        23 => call_with(day23::run, func),
        24 => call_with(day24::run, func),
        25 => call_with(day25::run, func),
        _ => {
            panic!("Invalid day number provided")
        }
    };
}

fn call_with<'a, T: Display + 'a, U: Display + 'a>(
    _run: impl Fn() -> (T, U),
    _func: impl Fn(&dyn Display, &dyn Display) + 'a,
) {
    #[cfg(not(test))]
    {
        let tuple = _run();
        _func(&tuple.0, &tuple.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_solutions_run_normally() {
        (1..=25).for_each(|day| run_with(day, |_, _| {}));
    }

    #[test]
    #[should_panic]
    fn test_invalid_solution_panic() {
        run_with(usize::MAX, |_, _| {});
    }
}
