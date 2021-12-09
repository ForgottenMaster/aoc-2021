use std::fs::read_to_string;

pub fn run() -> (u32, u32) {
    let input = read_to_string("input/day09.txt").expect("Can't read contents of input file.");
    let part_1 = low_points_from_input_string(&input)
        .into_iter()
        .map(|elem| (elem + 1) as u32)
        .sum();
    (part_1, 0)
}

/// Takes the input string of lines and returns the low points of the grid.
fn low_points_from_input_string(string: &str) -> Vec<u8> {
    let (grid, pitch) = extract_numeric_grid(string);
    low_point_iter(&grid, pitch).collect()
}

/// Takes a string slice and parses it to extract the numeric grid from it.
/// The numeric grid is a 2D grid where the pitch (length of a row) is the length
/// of a line. The system assumes that each line is the same length as it compresses
/// the grid into a single contiguous allocation in memory. If the input string does
/// not have lines of equal length this will crash (rather than bothering with actual
/// error handling).
fn extract_numeric_grid(string: &str) -> (Vec<u8>, usize) {
    let mut line_length = 0;
    (
        string
            .trim()
            .lines()
            .flat_map(|line| {
                let line = line.trim();
                let len = line.chars().count();

                if line_length == 0 {
                    line_length = line.chars().count();
                } else if line_length != len {
                    panic!("Input string has unequal line lengths.");
                }

                line.trim().chars().map(|c| {
                    c.to_digit(10)
                        .expect("Could not parse an input character as a u8.")
                        as u8
                })
            })
            .collect(),
        line_length,
    )
}

/// Function to convert from a 2D coordinate into a 1D index, given the pitch/length
/// of a row in the grid.
fn coordinate_to_index(pitch: usize, coordinate: (usize, usize)) -> usize {
    coordinate.1 * pitch + coordinate.0
}

/// Function to convert from a 1D index into a 2D coordinate, given the pitch/length
/// of a row in the grid.
fn index_to_coordinate(pitch: usize, index: usize) -> (usize, usize) {
    (index % pitch, index / pitch)
}

/// Produces an iterator over all the low points in the grid which can then be used to calculate
/// the number of low points or whatever.
fn low_point_iter(grid: &[u8], pitch: usize) -> impl Iterator<Item = u8> + '_ {
    let max_y = index_to_coordinate(pitch, grid.len() - 1).1;
    let max_x = pitch - 1;

    grid.into_iter().enumerate().filter_map(move |(idx, elem)| {
        let (x, y) = index_to_coordinate(pitch, idx);

        if (y > 0 && grid[coordinate_to_index(pitch, (x, y - 1))] <= *elem)
            || (y < max_y && grid[coordinate_to_index(pitch, (x, y + 1))] <= *elem)
            || (x > 0 && grid[coordinate_to_index(pitch, (x - 1, y))] <= *elem)
            || (x < max_x && grid[coordinate_to_index(pitch, (x + 1, y))] <= *elem)
        {
            None
        } else {
            Some(*elem)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_numeric_grid_empty() {
        const INPUT: &str = r#"
        


        "#;
        let expected = (vec![], 0);
        assert_eq!(extract_numeric_grid(INPUT), expected);
    }

    #[test]
    #[should_panic]
    fn test_extract_numeric_grid_invalid_char() {
        const INPUT: &str = r#"
        123456
        789o12
        "#;
        extract_numeric_grid(INPUT);
    }

    #[test]
    #[should_panic]
    fn test_extract_numeric_grid_unequal_lines() {
        const INPUT: &str = r#"
        123456
        789
        "#;
        extract_numeric_grid(INPUT);
    }

    #[test]
    fn test_extract_numeric_grid_success() {
        const INPUT: &str = r#"
        
        12345678
        87654321
        
        "#;
        let expected = (
            (1..=8).into_iter().chain((1..=8).rev()).collect::<Vec<_>>(),
            8,
        );
        assert_eq!(extract_numeric_grid(INPUT), expected);
    }

    #[test]
    fn test_index_to_coordinate() {
        const INDEX: usize = 7;
        const PITCH: usize = 3;
        const EXPECTED: (usize, usize) = (1, 2);
        assert_eq!(index_to_coordinate(PITCH, INDEX), EXPECTED);
    }

    #[test]
    fn test_index_to_coordinate_extremity() {
        const INDEX: usize = 2;
        const PITCH: usize = 3;
        const EXPECTED: (usize, usize) = (2, 0);
        assert_eq!(index_to_coordinate(PITCH, INDEX), EXPECTED);
    }

    #[test]
    fn test_coordinate_to_index() {
        const COORDINATE: (usize, usize) = (1, 1);
        const PITCH: usize = 3;
        const EXPECTED: usize = 4;
        assert_eq!(coordinate_to_index(PITCH, COORDINATE), EXPECTED);
    }

    #[test]
    fn test_coordinate_index_conversion_commutitive() {
        const INPUT: (usize, usize) = (17, 24);
        const PITCH: usize = 20;
        assert_eq!(
            index_to_coordinate(PITCH, coordinate_to_index(PITCH, INPUT)),
            INPUT
        );
    }

    #[test]
    fn test_low_point_iter() {
        const INPUT: &[u8] = &[2, 1, 9, 3, 9, 8, 9, 8, 5];
        const PITCH: usize = 3;
        let expected = vec![1, 5];
        assert_eq!(low_point_iter(&INPUT, PITCH).collect::<Vec<_>>(), expected);
    }

    #[test]
    fn test_low_points_from_input_string() {
        const INPUT: &str = r#"
        2199943210
        3987894921
        9856789892
        8767896789
        9899965678
        "#;
        let expected = vec![1, 0, 5, 5];
        assert_eq!(low_points_from_input_string(INPUT), expected);
    }
}
