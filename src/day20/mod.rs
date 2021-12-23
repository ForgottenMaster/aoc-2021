mod algorithm;
mod image;

use {crate::common::iter::FilterGroupMapExt, algorithm::Algorithm, image::Image};

pub fn run(input: &str) -> (usize, usize) {
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
    (0..2).for_each(|_| image = algorithm.apply(&image));
    let part_1 = image.pixels().filter(|(_, _, pixel)| **pixel).count();

    // For part 2, we need to do another 48 times.
    (0..48).for_each(|_| image = algorithm.apply(&image));
    let part_2 = image.pixels().filter(|(_, _, pixel)| **pixel).count();

    (part_1, part_2)
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
        const EXPECTED: (usize, usize) = (35, 3351);
        assert_eq!(run(INPUT), EXPECTED);
    }
}
