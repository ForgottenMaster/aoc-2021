mod algorithm;
mod image;

use {crate::common::iter::FilterGroupMapExt, algorithm::Algorithm, image::Image};

pub fn run(input: &str) -> (usize, u32) {
    // Regroup lines into two groups, one for the algorithm, and one for the image.
    let mut iter = input.trim().lines().filter_group_map(
        |line| !line.trim().is_empty(),
        |lines| {
            lines
                .into_iter()
                .map(|elem| *elem)
                .collect::<Vec<_>>()
                .join("\n")
        },
    );

    // Parse the algorithm and image.
    let algorithm = iter.next().unwrap().parse::<Algorithm>().unwrap();
    let mut image = iter.next().unwrap().parse::<Image>().unwrap();

    // Apply the enhancement algorithm to the image twice.
    image = algorithm.apply(&algorithm.apply(&image));

    // Part 1 answer is just the count of lit pixels.
    let part_1 = image.pixels().filter(|(_, _, pixel)| **pixel).count();

    (part_1, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        const INPUT: &str = "
        ..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
        #..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###
        .######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.
        .#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....
        .#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..
        ...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....
        ..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

        #..#.
        #....
        ##..#
        ..#..
        ..###
        ";
        const EXPECTED: (usize, u32) = (35, 0);
        assert_eq!(run(INPUT), EXPECTED);
    }
}
