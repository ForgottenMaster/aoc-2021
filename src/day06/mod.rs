mod lanternfish;

use {
    lanternfish::Lanternfish,
    std::{
        error::Error,
        fmt::Display,
        fs::File,
        io::{BufRead, BufReader},
        num::ParseIntError,
    },
};

const RESET_TO: u8 = 6;
const SPAWN_AT: u8 = 8;

pub fn run() -> Result<(Box<dyn Display>, Box<dyn Display>), Box<dyn Error>> {
    let file = File::open("input/day06.txt")?;
    let reader = BufReader::new(file);
    let mut new_fish_temporary_storage = vec![];
    let mut lanternfish = read_lanternfish(reader)?;
    simulate_days(&mut lanternfish, &mut new_fish_temporary_storage, 80);
    let part_1 = lanternfish.len();
    simulate_days(&mut lanternfish, &mut new_fish_temporary_storage, 176); // part 2 is 176 iterations more than part 1
    let part_2 = lanternfish.len();
    Ok((Box::new(part_1), Box::new(part_2)))
}

fn read_lanternfish(reader: impl BufRead) -> Result<Vec<Lanternfish>, ParseIntError> {
    let mut fish = Vec::new();
    for line in reader.lines().filter_map(|line| Some(line.ok()?)) {
        for elem in line.trim().split(",") {
            fish.push(elem.trim().parse::<u8>()?.into());
        }
    }
    Ok(fish)
}

fn simulate_one_day(fish: &mut Vec<Lanternfish>, new_fish: &mut Vec<Lanternfish>) {
    new_fish.extend(
        fish.into_iter()
            .filter_map(|fish| fish.tick(RESET_TO, SPAWN_AT)),
    );
    fish.extend(new_fish.drain(..));
}

fn simulate_days(fish: &mut Vec<Lanternfish>, new_fish: &mut Vec<Lanternfish>, days: u32) {
    (0..days).for_each(|_| simulate_one_day(fish, new_fish));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_lanternfish_empty() {
        const INPUT: &[u8] = "".as_bytes();
        let expected = vec![];
        assert_eq!(read_lanternfish(INPUT).unwrap(), expected);
    }

    #[test]
    fn test_read_lanternfish_single() {
        const INPUT: &[u8] = "245".as_bytes();
        let expected = vec![245.into()];
        assert_eq!(read_lanternfish(INPUT).unwrap(), expected);
    }

    #[test]
    fn test_read_lanternfish_multi() {
        const INPUT: &[u8] = "245, 106".as_bytes();
        let expected = vec![245.into(), 106.into()];
        assert_eq!(read_lanternfish(INPUT).unwrap(), expected);
    }

    #[test]
    fn test_simulate_1_day() {
        const INPUT: &[u8] = "3, 4, 3, 1, 2".as_bytes();
        const EXPECTED: &[u8] = "2, 3, 2, 0, 1".as_bytes();
        let mut input = read_lanternfish(INPUT).unwrap();
        let expected = read_lanternfish(EXPECTED).unwrap();
        let mut new_fish = vec![];
        simulate_days(&mut input, &mut new_fish, 1);
        assert_eq!(input, expected);
    }

    #[test]
    fn test_simulate_5_days() {
        const INPUT: &[u8] = "3, 4, 3, 1, 2".as_bytes();
        const EXPECTED: &[u8] = "5, 6, 5, 3, 4, 5, 6, 7, 7, 8".as_bytes();
        let mut input = read_lanternfish(INPUT).unwrap();
        let expected = read_lanternfish(EXPECTED).unwrap();
        let mut new_fish = vec![];
        simulate_days(&mut input, &mut new_fish, 5);
        assert_eq!(input, expected);
    }
}
