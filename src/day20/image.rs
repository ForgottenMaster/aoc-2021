use std::{convert::Infallible, iter::repeat, str::FromStr};

/// Represents the 2D grid of pixels. We'll just store this as
/// an actual 2D grid, rather than flattening it, since we're going
/// to want to access by x/y coordinate anyway.
#[derive(Clone, Debug, PartialEq)]
pub struct Image {
    grid: Vec<Vec<bool>>,
    width: usize,
    height: usize,
    pub current_out_of_bounds_pixel_value: bool,
}

impl Image {
    /// Provides an iterator over the pixels of the image providing mutable access.
    pub fn pixels_mut(&mut self) -> impl Iterator<Item = (usize, usize, &mut bool)> {
        self.grid.iter_mut().enumerate().flat_map(|(y, row)| {
            row.iter_mut()
                .enumerate()
                .map(move |(x, elem)| (x, y, elem))
        })
    }

    /// Provides an iterator over the pixels of the image providing immutable access.
    pub fn pixels(&self) -> impl Iterator<Item = (usize, usize, &bool)> {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, elem)| (x, y, elem)))
    }

    /// Since the output image is infinite, we need to account for the
    /// setting of pixels which are off the edges of the input image. Since the
    /// algorithm works in a 3x3 grid around a given pixel, we only need a 1 pixel
    /// wide border.
    pub fn add_borders(&mut self) {
        self.width += 2;
        self.height += 2;
        self.grid.iter_mut().for_each(|line| {
            line.insert(0, self.current_out_of_bounds_pixel_value);
            line.insert(line.len(), self.current_out_of_bounds_pixel_value);
        });
        self.grid.insert(
            0,
            repeat(self.current_out_of_bounds_pixel_value)
                .take(self.width)
                .collect(),
        );
        self.grid.insert(
            self.grid.len(),
            repeat(self.current_out_of_bounds_pixel_value)
                .take(self.width)
                .collect(),
        );
    }

    /// Takes an input coordinate, where (0, 0) is the top left of the input image
    /// before getting to the infinite empty space. Note that the coordinate can go negative
    /// as the output image will be calculating pixels that are in the border of the image
    /// that doesn't exist in the input image.
    pub fn calculate_algorithm_index(&self, x: isize, y: isize) -> usize {
        (self.calculate_pixel_value(x - 1, y - 1) << 8)
            + (self.calculate_pixel_value(x, y - 1) << 7)
            + (self.calculate_pixel_value(x + 1, y - 1) << 6)
            + (self.calculate_pixel_value(x - 1, y) << 5)
            + (self.calculate_pixel_value(x, y) << 4)
            + (self.calculate_pixel_value(x + 1, y) << 3)
            + (self.calculate_pixel_value(x - 1, y + 1) << 2)
            + (self.calculate_pixel_value(x, y + 1) << 1)
            + self.calculate_pixel_value(x + 1, y + 1)
    }

    /// Calculates the value of the single pixel at the given coordinates. If the coordinate
    /// is off the image then it will be the current out of bounds pixel value.
    /// If it's on the image, it'll be whatever the value of the pixel is at that coordinate.
    fn calculate_pixel_value(&self, x: isize, y: isize) -> usize {
        if x < 0 || x as usize >= self.width || y < 0 || y as usize >= self.height {
            self.current_out_of_bounds_pixel_value as usize
        } else {
            self.grid[y as usize][x as usize] as usize
        }
    }
}

impl FromStr for Image {
    type Err = Infallible;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut grid = vec![];
        let mut width = 0;

        string.trim().lines().for_each(|line| {
            let line = line.trim();
            let line_len = line.len();
            if width != 0 && line_len != width {
                panic!(
                    "Input has unequal line lengths. Expected line length {}, found line length {}",
                    width, line_len
                );
            }
            width = line_len;
            grid.push(
                line.chars()
                    .map(|c| match c {
                        '#' => true,
                        '.' => false,
                        _ => panic!("Invalid character '{}' found in input string.", c),
                    })
                    .collect(),
            );
        });
        let height = grid.len();

        Ok(Self {
            grid,
            width,
            height,
            current_out_of_bounds_pixel_value: false,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_from_str() {
        const INPUT: &str = "
        #..#.
        #....
        ##..#
        ..#..
        ..###";
        let expected = Image {
            grid: vec![
                vec![true, false, false, true, false],
                vec![true, false, false, false, false],
                vec![true, true, false, false, true],
                vec![false, false, true, false, false],
                vec![false, false, true, true, true],
            ],
            width: 5,
            height: 5,
            current_out_of_bounds_pixel_value: false,
        };
        let calculated = INPUT.parse::<Image>().unwrap();
        assert_eq!(calculated, expected);
    }

    #[test]
    #[should_panic]
    fn test_image_from_str_invalid_char() {
        const INPUT: &str = "
        #..#.
        #....
        ##..#
        ..#0.
        ..###";
        let _ = INPUT.parse::<Image>().unwrap();
    }

    #[test]
    #[should_panic]
    fn test_image_from_str_unequal_line_lengths() {
        const INPUT: &str = "
        #..#.
        #....
        ##..#
        ..#..
        ..####";
        let _ = INPUT.parse::<Image>().unwrap();
    }

    #[test]
    fn test_image_add_borders() {
        let mut input = r#"
        #..#.
        #....
        ##..#
        ..#..
        ..###
        "#
        .parse::<Image>()
        .unwrap();
        let expected = r#"
        .......
        .#..#..
        .#.....
        .##..#.
        ...#...
        ...###.
        .......
        "#
        .parse::<Image>()
        .unwrap();
        input.add_borders();
        assert_eq!(input, expected);
    }

    #[test]
    fn test_calculate_pixel_value() {
        let input = r#"
        #..#.
        #....
        ##..#
        ..#..
        ..###
        "#
        .parse::<Image>()
        .unwrap();
        assert_eq!(input.calculate_pixel_value(-1, 3), 0);
        assert_eq!(input.calculate_pixel_value(5, 2), 0);
        assert_eq!(input.calculate_pixel_value(0, -1), 0);
        assert_eq!(input.calculate_pixel_value(0, 5), 0);
        assert_eq!(input.calculate_pixel_value(0, 2), 1);
        assert_eq!(input.calculate_pixel_value(0, 3), 0);
    }

    #[test]
    fn test_calculate_algorithm_index() {
        let input = r#"
        #..#.
        #....
        ##..#
        ..#..
        ..###
        "#
        .parse::<Image>()
        .unwrap();
        const EXPECTED: usize = 34;
        assert_eq!(input.calculate_algorithm_index(2, 2), EXPECTED);
    }

    #[test]
    fn test_pixels_mut() {
        let mut input = r#"
        #..#.
        #....
        ##..#
        ..#..
        ..###
        "#
        .parse::<Image>()
        .unwrap();
        let expected = r#"
        .##.#
        .####
        ..##.
        ##.##
        ##...
        "#
        .parse::<Image>()
        .unwrap();
        input.pixels_mut().for_each(|(_, _, elem)| *elem = !*elem);
        assert_eq!(input, expected);
    }

    #[test]
    fn test_pixels() {
        let input = r#"
        #..#.
        #....
        ##..#
        ..#..
        ..###
        "#
        .parse::<Image>()
        .unwrap();
        let count = input.pixels().filter(|(_, _, elem)| **elem).count();
        assert_eq!(count, 10);
    }
}
