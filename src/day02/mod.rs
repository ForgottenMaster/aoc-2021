mod command;
mod direction;
mod folder;
mod folder_composite;
mod folder_part_1;
mod folder_part_2;

use {
    command::Command,
    folder::Folder,
    folder_composite::FolderComposite,
    folder_part_1::FolderPart1,
    folder_part_2::FolderPart2,
    std::{
        error::Error,
        fmt::Display,
        fs::File,
        io::{BufRead, BufReader},
    },
};

pub fn run() -> Result<(Box<dyn Display>, Box<dyn Display>), Box<dyn Error>> {
    let file = File::open("input/day02.txt")?;
    let reader = BufReader::new(file);
    let (part_1, part_2) = calculate_distance_travelled(reader, FolderBoth::default());
    Ok((Box::new(part_1), Box::new(part_2)))
}

/// Runs the puzzle over the given buffered reader and uses the provided
/// "Folder" for the fold step which allows the caller to run part1 logic, part_2
/// logic, or both (additionally if we wanted to apply some hypothetical other interpretation we could).
fn calculate_distance_travelled<F: Folder>(reader: impl BufRead, folder: F) -> F::Output {
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

type FolderBoth = FolderComposite<FolderPart1, FolderPart2>;

#[cfg(test)]
mod tests {
    use super::*;

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
        let calculated = calculate_distance_travelled(INPUT, FolderPart1::default());
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
        let calculated = calculate_distance_travelled(INPUT, FolderPart2::default());
        assert_eq!(calculated, EXPECTED);
    }
}
