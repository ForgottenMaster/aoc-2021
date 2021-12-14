use std::{collections::HashSet, fs::read_to_string};

#[derive(Debug, PartialEq)]
pub enum ExecutionError {}

pub fn run() -> (u32, u32) {
    let input = read_to_string("input/day09.txt").expect("Can't read contents of input file.");
    let (grid, pitch) = extract_numeric_grid(&input);
    let low_points = extract_low_points(&grid, pitch);
    let part_1 = low_points.iter().map(|(elem, _)| (elem + 1) as u32).sum();
    let mut flooded_coordinates = HashSet::with_capacity(grid.len()); // worst case is that the entire grid is flooded (not the case though).
    let mut basin_sizes = low_points
        .iter()
        .map(|(_, coord)| calculate_basin_size(*coord, &grid, pitch, &mut flooded_coordinates))
        .collect::<Vec<_>>();
    basin_sizes.sort_by(|a, b| b.cmp(a));
    let part_2 = basin_sizes.into_iter().take(3).product();
    (part_1, part_2)
}

/// Calculates the size of a basin flooding from a given starting coordinate. Since
/// each coordinate only appears in exactly one basin, we can pass around an allocated HashSet
/// to use as storage so we know if a coordinate has been visited before (so we don't flood over it again).
/// Doing this as a simple recursive function should be good enough.
fn calculate_basin_size(
    coord: (usize, usize),
    grid: &[u8],
    pitch: usize,
    flooded: &mut HashSet<(usize, usize)>,
) -> u32 {
    if grid[coordinate_to_index(pitch, coord)] == 9 || !flooded.insert(coord) {
        0
    } else {
        let (x, y) = coord;
        let max_x = pitch - 1;
        let max_y = (grid.len() / pitch) - 1;
        let mut count = 1;

        // add the sizes reported from neighbours.
        if x > 0 {
            count += calculate_basin_size((x - 1, y), grid, pitch, flooded);
        }

        if x < max_x {
            count += calculate_basin_size((x + 1, y), grid, pitch, flooded);
        }

        if y > 0 {
            count += calculate_basin_size((x, y - 1), grid, pitch, flooded);
        }

        if y < max_y {
            count += calculate_basin_size((x, y + 1), grid, pitch, flooded);
        }

        count
    }
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

/// Takes the grid and pitch of the grid and determines where the low points in the grid are.
/// Returns the low points themselves along with the 2D coordinates to find them in the grid.
fn extract_low_points(grid: &[u8], pitch: usize) -> Vec<(u8, (usize, usize))> {
    let max_y = (grid.len() / pitch) - 1;
    let max_x = pitch - 1;

    grid.into_iter()
        .enumerate()
        .filter_map(move |(idx, elem)| {
            let (x, y) = index_to_coordinate(pitch, idx);

            if (y > 0 && grid[coordinate_to_index(pitch, (x, y - 1))] <= *elem)
                || (y < max_y && grid[coordinate_to_index(pitch, (x, y + 1))] <= *elem)
                || (x > 0 && grid[coordinate_to_index(pitch, (x - 1, y))] <= *elem)
                || (x < max_x && grid[coordinate_to_index(pitch, (x + 1, y))] <= *elem)
            {
                None
            } else {
                Some((*elem, index_to_coordinate(pitch, idx)))
            }
        })
        .collect()
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
    fn test_low_point_extraction() {
        const INPUT: &[u8] = &[2, 1, 9, 3, 9, 8, 9, 8, 5];
        const PITCH: usize = 3;
        let expected = vec![(1, (1, 0)), (5, (2, 2))];
        assert_eq!(extract_low_points(&INPUT, PITCH), expected);
    }

    #[test]
    fn test_calculate_basin_size() {
        const INPUT: &str = r#"
        2199943210
        3987894921
        9856789892
        8767896789
        9899965678
        "#;
        let (grid, pitch) = extract_numeric_grid(INPUT);
        let mut hs = HashSet::with_capacity(grid.len());
        assert_eq!(calculate_basin_size((0, 0), &grid, pitch, &mut hs), 3);
        assert_eq!(calculate_basin_size((9, 0), &grid, pitch, &mut hs), 9);
        assert_eq!(calculate_basin_size((2, 1), &grid, pitch, &mut hs), 14);
        assert_eq!(calculate_basin_size((9, 4), &grid, pitch, &mut hs), 9);
    }
}
