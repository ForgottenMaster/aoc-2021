mod grid;

use grid::Grid;

pub fn run(input: &str) -> (u32, u32) {
    let mut grid: Grid<Vec<_>> = input.trim().lines().into();
    let part_1 = grid.run_step_count(100);
    let part_2 = grid.run_until_synchronized_flash();
    (part_1, part_2)
}
