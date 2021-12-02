use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
    time::Instant,
};

fn main() -> std::io::Result<()> {
    let start_time = Instant::now();
    let file = File::open("input/day02.txt")?;
    let reader = BufReader::new(file);
    println!("Part 1 => {}", run(reader));
    let end_time = Instant::now();
    let duration = end_time.duration_since(start_time);
    println!("Took {} nanoseconds to run", duration.as_nanos());
    Ok(())
}

fn run(reader: impl BufRead) -> u32 {
    let (horizontal_distance, depth) = reader
        .lines()
        .filter_map(|elem| {
            let line = elem.ok()?;
            let parsed: Command = line.trim().parse().ok()?;
            Some(parsed)
        })
        .fold((0, 0), |state, elem| elem.apply(state));
    horizontal_distance * depth
}

#[derive(Debug)]
enum Direction {
    Forward,
    Down,
    Up,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string.trim() {
            "forward" => Ok(Direction::Forward),
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Command {
    direction: Direction,
    amount: u32,
}

impl Command {
    fn apply(&self, input: (u32, u32)) -> (u32, u32) {
        match self.direction {
            Direction::Forward => (input.0 + self.amount, input.1),
            Direction::Down => (input.0, input.1 + self.amount),
            Direction::Up => (
                input.0,
                if input.1 >= self.amount {
                    input.1 - self.amount
                } else {
                    0
                },
            ), // can't go higher than 0
        }
    }
}

impl FromStr for Command {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut split = string.trim().split(" ");
        let direction_string = split.next().ok_or(())?;
        let amount_string = split.next().ok_or(())?;
        let direction = direction_string.parse()?;
        let amount = amount_string.parse().map_err(|_| ())?;

        Ok(Command { direction, amount })
    }
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn test_example_part_1() {
        const INPUT: &[u8] = r#"
        forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2
        "#
        .as_bytes();
        const EXPECTED: u32 = 150;
        let calculated = run(INPUT);
        assert_eq!(calculated, EXPECTED);
    }
}
