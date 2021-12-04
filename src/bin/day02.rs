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
    let (part_1, part_2) = run(reader, FolderBoth::default());
    println!("Part 1 => {}", part_1);
    println!("Part 2 => {}", part_2);
    let end_time = Instant::now();
    let duration = end_time.duration_since(start_time);
    println!("Took {} microseconds to run", duration.as_micros());
    Ok(())
}

/// Runs the puzzle over the given buffered reader and uses the provided
/// "Folder" for the fold step which allows the caller to run part1 logic, part_2
/// logic, or both (additionally if we wanted to apply some hypothetical other interpretation we could).
fn run<F: Folder>(reader: impl BufRead, folder: F) -> F::Output {
    reader
        .lines()
        .filter_map(|elem| {
            let line = elem.ok()?;
            let parsed: Command = line.trim().parse().ok()?;
            Some(parsed)
        })
        .fold(folder, |state, elem| state.apply(elem))
        .unwrap()
}

///////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
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

//////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
struct Command {
    direction: Direction,
    amount: u32,
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

//////////////////////////////////////////////////////////////////////////////////////////////////

// Here we put the "folders" which are the things that determine how the run function processes the input
// letting us get either the part 1 interpretation of commands, part 2 interpretation, or both which lets
// us run both parts in a single pass. Using a trait to define this may be a good practice in "real code"
// as it will allow calling the run function with whatever interpretation of the commands you need to use.
trait Folder {
    type Output;

    fn apply(self, command: Command) -> Self;
    fn unwrap(self) -> Self::Output;
}

#[derive(Default)]
struct FolderPart1 {
    horizontal: u32,
    depth: u32,
}

impl Folder for FolderPart1 {
    type Output = u32;

    fn apply(self, command: Command) -> Self {
        match command.direction {
            Direction::Forward => Self {
                horizontal: self.horizontal + command.amount,
                depth: self.depth,
            },
            Direction::Down => Self {
                horizontal: self.horizontal,
                depth: self.depth + command.amount,
            },
            Direction::Up => Self {
                horizontal: self.horizontal,
                depth: if self.depth >= command.amount {
                    self.depth - command.amount
                } else {
                    0
                },
            },
        }
    }

    fn unwrap(self) -> Self::Output {
        self.horizontal * self.depth
    }
}

#[derive(Default)]
struct FolderPart2 {
    horizontal: u32,
    depth: u32,
    aim: u32,
}

impl Folder for FolderPart2 {
    type Output = u32;

    fn apply(self, command: Command) -> Self {
        match command.direction {
            Direction::Down => Self {
                horizontal: self.horizontal,
                depth: self.depth,
                aim: self.aim + command.amount,
            },
            Direction::Up => Self {
                horizontal: self.horizontal,
                depth: self.depth,
                aim: if self.aim >= command.amount {
                    self.aim - command.amount
                } else {
                    0
                },
            },
            Direction::Forward => Self {
                horizontal: self.horizontal + command.amount,
                depth: self.depth + self.aim * command.amount,
                aim: self.aim,
            },
        }
    }

    fn unwrap(self) -> Self::Output {
        self.horizontal * self.depth
    }
}

#[derive(Default)]
struct FolderComposite<Folder1, Folder2> {
    folder_1: Folder1,
    folder_2: Folder2,
}
type FolderBoth = FolderComposite<FolderPart1, FolderPart2>;

impl<Folder1: Folder, Folder2: Folder> Folder for FolderComposite<Folder1, Folder2> {
    type Output = (Folder1::Output, Folder2::Output);

    fn apply(self, command: Command) -> Self {
        Self {
            folder_1: self.folder_1.apply(command.clone()),
            folder_2: self.folder_2.apply(command),
        }
    }

    fn unwrap(self) -> Self::Output {
        (self.folder_1.unwrap(), self.folder_2.unwrap())
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::{run, FolderPart1, FolderPart2};

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
        let calculated = run(INPUT, FolderPart1::default());
        assert_eq!(calculated, EXPECTED);
    }

    #[test]
    fn test_example_part_2() {
        const INPUT: &[u8] = r#"
        forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2
        "#
        .as_bytes();
        const EXPECTED: u32 = 900;
        let calculated = run(INPUT, FolderPart2::default());
        assert_eq!(calculated, EXPECTED);
    }
}
