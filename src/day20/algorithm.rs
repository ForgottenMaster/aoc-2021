use {
    super::Image,
    std::{convert::Infallible, str::FromStr},
};

/// Structure representing the 512 bit image enhancement algorithm
/// which is used on the grid to enhance the image.
#[derive(Debug, PartialEq)]
pub struct Algorithm([bool; 512]);

impl Algorithm {
    /// Applies this enhancement algorithm to a given image to produce a new
    /// image.
    pub fn apply(&self, image: &Image) -> Image {
        let mut output = (*image).clone(); // clone image as a starting point (to get correct dimensions)
        output.add_borders();

        // On an infinite image, if we parse a pixel that is entirely using the "out of bounds" value then it's either
        // 0 (all off) or 512 (all on). We can therefore easily decide what the rest of the pixels on the infinite sheet will be.
        output.current_out_of_bounds_pixel_value = if output.current_out_of_bounds_pixel_value {
            self.0[511]
        } else {
            self.0[0]
        };

        output.pixels_mut().for_each(|(x, y, elem)| {
            // remap x and y from {0, image.width+2} range to {-1, image.width+1} range.
            // this just means subtracting 1.
            let x = (x as isize) - 1;
            let y = (y as isize) - 1;
            *elem = self.0[image.calculate_algorithm_index(x, y)];
        });
        output
    }
}

impl FromStr for Algorithm {
    type Err = Infallible;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut arr = [false; 512];
        string
            .trim()
            .chars()
            .filter_map(|c| match c {
                '.' => Some(false),
                '#' => Some(true),
                _ => None,
            })
            .enumerate()
            .for_each(|(idx, elem)| {
                arr[idx] = elem;
            });
        Ok(Self(arr))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_algorithm_from_str() {
        const INPUT: &str = "
        ..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
        #..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###
        .######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.
        .#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....
        .#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..
        ...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....
        ..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#";
        let mut arr = [false; 512];
        INPUT
            .trim()
            .chars()
            .filter_map(|c| match c {
                '.' => Some(false),
                '#' => Some(true),
                _ => None,
            })
            .enumerate()
            .for_each(|(idx, elem)| {
                arr[idx] = elem;
            });
        let calculated = INPUT.parse::<Algorithm>().unwrap();
        let expected = Algorithm(arr);
        assert_eq!(calculated, expected);
    }

    #[test]
    fn test_algorithm_apply() {
        let algorithm = "
        ..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
        #..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###
        .######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.
        .#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....
        .#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..
        ...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....
        ..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#
        "
        .parse::<Algorithm>()
        .unwrap();
        let image = "
        #..#.
        #....
        ##..#
        ..#..
        ..###
        "
        .parse::<Image>()
        .unwrap();
        let output = "
        .##.##.
        #..#.#.
        ##.#..#
        ####..#
        .#..##.
        ..##..#
        ...#.#.
        "
        .parse::<Image>()
        .unwrap();
        assert_eq!(algorithm.apply(&image), output);
    }
}
