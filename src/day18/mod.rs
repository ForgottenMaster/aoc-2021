mod number;

use number::Number;

pub fn run(input: &str) -> (u64, u32) {
    let mut stack = vec![];
    let mut iter = input.trim().lines();
    let total = iter.next().unwrap().trim().parse::<Number>().unwrap();
    let total = iter.fold(total, |total, line| {
        let total = total + line.trim().parse::<Number>().unwrap();
        total.reduce(&mut stack);
        total
    });
    let part_1 = total.magnitude();
    (part_1, 0)
}
