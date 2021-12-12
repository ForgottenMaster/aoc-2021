mod grid;

use {grid::Grid, std::fs::read_to_string};

pub fn run() -> (u32, u32) {
    let content = read_to_string("input/day11.txt").expect("Couldn't read from file.");
    let mut grid: Grid<Vec<_>> = content.trim().lines().into();
    let part_1 = grid.run_step_count(100);
    (part_1, 0)
}
