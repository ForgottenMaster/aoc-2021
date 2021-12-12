use {crate::common::collections::Stack, std::borrow::Borrow};

/// Represents the grid of octopuses and allows us to have a strong type
/// that can only be constructed from valid input. If we have an instance
/// of this type then we know that it's a grid and all lines are equal length.
pub struct Grid<S> {
    grid: Vec<u8>,
    flash_stack: S,
    width: usize,
    height: usize,
    flash_count: u32,
}

impl<S: Stack<(usize, usize)>> Grid<S> {
    /// Runs X iterations of the loop and returns the number of flashes.
    pub fn run_step_count(&mut self, count: u32) -> u32 {
        self.flash_count = 0;
        (0..count).for_each(|_| self.run_single_step());
        self.flash_count
    }

    /// Runs a single time step of the energy increase + flash and
    /// returns the number of cells that flashed.
    fn run_single_step(&mut self) {
        self.increase_energy_levels();
        self.process_flash_stack();
        self.reset_flashed_energy_levels();
    }

    /// Increases the energy levels in the first step of the process. This
    /// can't increase energy levels past 10 (10 indicates it should flash).
    /// Pushes any that are turning 10 onto the flash_stack for flashing.
    fn increase_energy_levels(&mut self) {
        self.grid.iter_mut().enumerate().for_each(|(idx, elem)| {
            if *elem < 10 {
                *elem += 1;
                if *elem == 10 {
                    self.flash_stack.push((idx % self.width, idx / self.width));
                    self.flash_count += 1;
                }
            }
        });
    }

    /// Processes the flash stack until it's empty. This could run through multiple
    /// iterations to process all the flashes.
    fn process_flash_stack(&mut self) {
        while let Some((x, y)) = self.flash_stack.pop() {
            // process top row neighbours
            if y > 0 {
                if x > 0 {
                    self.try_flash((x - 1, y - 1));
                }
                self.try_flash((x, y - 1));
                self.try_flash((x + 1, y - 1));
            }

            // Process same row neighbours
            if x > 0 {
                self.try_flash((x - 1, y));
            }
            self.try_flash((x + 1, y));

            // Process bottom row neigbours
            if x > 0 {
                self.try_flash((x - 1, y + 1));
            }
            self.try_flash((x, y + 1));
            self.try_flash((x + 1, y + 1));
        }
    }

    /// Resets all the energy level 10 cells back to 0.
    fn reset_flashed_energy_levels(&mut self) {
        self.grid.iter_mut().for_each(|elem| {
            if *elem == 10 {
                *elem = 0;
            }
        });
    }

    /// Function which increments the element at the given coordinate if it's less than 10, and then
    /// if it hits 10, pushes it onto the flash stack for processing.
    fn try_flash(&mut self, coord: (usize, usize)) {
        let (x, y) = coord;
        if x < self.width && y < self.height {
            let idx = y * self.width + x;
            if self.grid[idx] < 10 {
                self.grid[idx] += 1;
                if self.grid[idx] == 10 {
                    self.flash_stack.push((x, y));
                    self.flash_count += 1;
                }
            }
        }
    }
}

/// Makes a grid from an iterator over some type that can be borrowed as a string slice (so it works with owned strings and also
/// with string slices). Panics if the iterator contains invalid data which could be non-numeric characters, or different sized line
/// lengths.
impl<S, I> From<I> for Grid<S>
where
    I: Iterator,
    <I as Iterator>::Item: Borrow<str>,
    S: Default + Stack<(usize, usize)>,
{
    fn from(iter: I) -> Self {
        let (width, grid) = iter.fold((0, vec![]), |(mut width, mut grid), line| {
            let line = line.borrow().trim();
            let line_len = line.len();

            if line_len != 0 {
                if width != 0 {
                    if line_len != width {
                        panic!("Mismatching line lengths detected. Expected a line of length {} but found length {}.", width, line_len);
                    }
                } else {
                    width = line_len;
                }
                grid.extend(line.chars().map(|c| c.to_digit(10).expect("Expected all characters to be valid numeric digits.") as u8));
            }
            (width, grid)
        });
        let height = grid.len() / width;
        let flash_stack = S::default();

        Self {
            grid,
            flash_stack,
            width,
            height,
            flash_count: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_from_lines_to_grid_invalid_bad_character() {
        const INPUT: &str = r#"
        123
        45e
        789
        "#;
        let _: Grid<Vec<_>> = INPUT.trim().lines().into();
    }

    #[test]
    #[should_panic]
    fn test_from_lines_to_grid_invalid_mismatching_line_lengths() {
        const INPUT: &str = r#"
        123
        4567
        890
        "#;
        let _: Grid<Vec<_>> = INPUT.trim().lines().into();
    }

    #[test]
    fn test_from_lines_to_grid_valid() {
        const INPUT: &str = r#"
        123
        456
        789
        "#;
        let grid: Grid<Vec<_>> = INPUT.trim().lines().into();
        assert_eq!(grid.grid.len(), 9);
        assert_eq!(grid.flash_stack.len(), 0);
        assert_eq!(grid.width, 3);
        assert_eq!(grid.height, 3);
    }

    #[test]
    fn test_increase_energy_levels() {
        const INPUT: &str = r#"
        123
        456
        789
        "#;
        let mut grid: Grid<Vec<_>> = INPUT.trim().lines().into();
        assert_eq!(grid.grid, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(grid.flash_stack, vec![]);
        grid.increase_energy_levels();
        assert_eq!(grid.grid, vec![2, 3, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(grid.flash_stack, vec![(2, 2)]);
        grid.increase_energy_levels();
        assert_eq!(grid.grid, vec![3, 4, 5, 6, 7, 8, 9, 10, 10]);
        assert_eq!(grid.flash_stack, vec![(2, 2), (1, 2)]);
    }

    #[test]
    fn test_try_flash_out_of_bounds() {
        const INPUT: &str = r#"
        123
        456
        789
        "#;
        let mut grid: Grid<Vec<_>> = INPUT.trim().lines().into();
        grid.try_flash((3, 1));
        assert_eq!(grid.grid, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(grid.flash_stack, vec![]);
        grid.try_flash((2, 3));
        assert_eq!(grid.grid, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(grid.flash_stack, vec![]);
    }

    #[test]
    fn test_try_flash_energy_level_10() {
        let grid = vec![1, 2, 3, 4, 5, 10, 7, 8, 9];
        let flash_stack = vec![];
        let width = 3;
        let height = 3;
        let mut grid = Grid {
            grid,
            flash_stack,
            width,
            height,
            flash_count: 0,
        };
        grid.try_flash((2, 1));
        assert_eq!(grid.grid, vec![1, 2, 3, 4, 5, 10, 7, 8, 9]);
        assert_eq!(grid.flash_stack, vec![]);
    }

    #[test]
    fn test_try_flash_energy_level_9() {
        const INPUT: &str = r#"
        123
        456
        789
        "#;
        let mut grid: Grid<Vec<_>> = INPUT.trim().lines().into();
        assert_eq!(grid.flash_stack, vec![]);
        grid.try_flash((2, 2));
        assert_eq!(grid.grid, vec![1, 2, 3, 4, 5, 6, 7, 8, 10]);
        assert_eq!(grid.flash_stack, vec![(2, 2)]);
    }

    #[test]
    fn test_try_flash_energy_level_6() {
        const INPUT: &str = r#"
        123
        456
        789
        "#;
        let mut grid: Grid<Vec<_>> = INPUT.trim().lines().into();
        assert_eq!(grid.flash_stack, vec![]);
        grid.try_flash((2, 1));
        assert_eq!(grid.grid, vec![1, 2, 3, 4, 5, 7, 7, 8, 9]);
        assert_eq!(grid.flash_stack, vec![]);
    }

    #[test]
    fn test_process_flash_stack() {
        const INPUT: &str = r#"
        11111
        19991
        19191
        19991
        11111
        "#;
        let mut grid: Grid<Vec<_>> = INPUT.trim().lines().into();
        let expected = vec![
            3, 4, 5, 4, 3, 4, 10, 10, 10, 4, 5, 10, 10, 10, 5, 4, 10, 10, 10, 4, 3, 4, 5, 4, 3,
        ];
        grid.increase_energy_levels();
        grid.process_flash_stack();
        assert_eq!(grid.grid, expected);
        assert_eq!(grid.flash_stack, vec![]);
    }

    #[test]
    fn test_reset_flashed_energy_levels() {
        const INPUT: &str = r#"
        11111
        19991
        19191
        19991
        11111
        "#;
        let mut grid: Grid<Vec<_>> = INPUT.trim().lines().into();
        let expected = vec![
            3, 4, 5, 4, 3, 4, 0, 0, 0, 4, 5, 0, 0, 0, 5, 4, 0, 0, 0, 4, 3, 4, 5, 4, 3,
        ];
        grid.increase_energy_levels();
        grid.process_flash_stack();
        grid.reset_flashed_energy_levels();
        assert_eq!(grid.grid, expected);
        assert_eq!(grid.flash_stack, vec![]);
    }

    #[test]
    fn test_run_single_step() {
        const INPUT: &str = r#"
        11111
        19991
        19191
        19991
        11111
        "#;
        let mut grid: Grid<Vec<_>> = INPUT.trim().lines().into();
        grid.run_single_step();
        assert_eq!(
            grid.grid,
            vec![3, 4, 5, 4, 3, 4, 0, 0, 0, 4, 5, 0, 0, 0, 5, 4, 0, 0, 0, 4, 3, 4, 5, 4, 3]
        );
        grid.run_single_step();
        assert_eq!(
            grid.grid,
            vec![4, 5, 6, 5, 4, 5, 1, 1, 1, 5, 6, 1, 1, 1, 6, 5, 1, 1, 1, 5, 4, 5, 6, 5, 4]
        );
    }

    #[test]
    fn test_run_step_count() {
        const INPUT: &str = r#"
        5483143223
        2745854711
        5264556173
        6141336146
        6357385478
        4167524645
        2176841721
        6882881134
        4846848554
        5283751526
        "#;
        let mut grid: Grid<Vec<_>> = INPUT.trim().lines().into();
        let flash_count = grid.run_step_count(100);
        assert_eq!(
            grid.grid,
            vec![
                0, 3, 9, 7, 6, 6, 6, 8, 6, 6, 0, 7, 4, 9, 7, 6, 6, 9, 1, 8, 0, 0, 5, 3, 9, 7, 6, 9,
                3, 3, 0, 0, 0, 4, 2, 9, 7, 8, 2, 2, 0, 0, 0, 4, 2, 2, 9, 8, 9, 2, 0, 0, 5, 3, 2, 2,
                2, 8, 7, 7, 0, 5, 3, 2, 2, 2, 2, 9, 6, 6, 9, 3, 2, 2, 2, 2, 8, 9, 6, 6, 7, 9, 2, 2,
                2, 8, 6, 8, 6, 6, 6, 7, 8, 9, 9, 9, 8, 7, 6, 6
            ]
        );
        assert_eq!(flash_count, 1656);
    }
}
