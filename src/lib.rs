mod common;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

use std::fmt::Display;

pub fn run_with(day: usize, func: impl Fn(&dyn Display, &dyn Display)) {
    match day {
        1 => call_with(day01::run(), func),
        2 => call_with(day02::run(), func),
        3 => call_with(day03::run(), func),
        4 => call_with(day04::run(), func),
        5 => call_with(day05::run(), func),
        6 => call_with(day06::run(), func),
        7 => call_with(day07::run(), func),
        _ => {
            panic!("Invalid day {} passed to run_with function", day);
        }
    };
}

fn call_with<'a>(
    tuple: (impl Display + 'a, impl Display + 'a),
    func: impl Fn(&dyn Display, &dyn Display) + 'a,
) {
    func(&tuple.0, &tuple.1)
}
