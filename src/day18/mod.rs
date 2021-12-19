mod number;

use number::Number;

pub fn run(input: &str) -> (u64, u64) {
    let mut stack = vec![];
    let numbers = input
        .trim()
        .lines()
        .map(|line| line.trim().parse::<Number>().unwrap())
        .collect::<Vec<_>>();
    let part_1 = numbers
        .iter()
        .fold(numbers[0].clone(), |total, number| {
            let total = total + (*number).clone();
            total.reduce(&mut stack);
            total
        })
        .magnitude();
    let part_2 = numbers
        .iter()
        .enumerate()
        .flat_map(|(i, number_i)| {
            numbers
                .iter()
                .take(i)
                .chain(numbers.iter().skip(i + 1))
                .map(|number_j| number_i.clone() + number_j.clone())
        })
        .map(|number| {
            number.reduce(&mut stack);
            number.magnitude()
        })
        .max()
        .unwrap();
    (part_1, part_2)
}
