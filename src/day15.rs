use std::{cmp::Reverse, iter::repeat};

pub fn run(input: &str) -> (u64, u64) {
    let (nodes, stride) = parse_input(input);
    let part_1 = find_shortest_path_cost(&nodes, stride);
    let (nodes, stride) = expand_grid(nodes, stride);
    let part_2 = find_shortest_path_cost(&nodes, stride);
    (part_1, part_2)
}

/// Takes the input string which should consist of equal length lines
/// and parses it as a 2D grid (returning it as a 1D grid and the line length/stride).
fn parse_input(input: &str) -> (Vec<u64>, usize) {
    input
        .trim()
        .lines()
        .fold((Vec::new(), 0), |(mut cells, mut stride), line| {
            let line = line.trim();
            if !line.is_empty() {
                let line_len = line.len();
                if stride == 0 {
                    stride = line_len;
                } else if line_len != stride {
                    panic!(
                        "Invalid line length, expected {stride}, found {line_len}",
                        stride = stride,
                        line_len = line_len
                    );
                }
                cells.extend(line.chars().map(|elem| elem.to_digit(10).unwrap() as u64));
            }
            (cells, stride)
        })
}

/// Expands the grid by 5 times in either direction producing a new grid that's based on the initial one
/// but with increased risk costs.
fn expand_grid(nodes: Vec<u64>, stride: usize) -> (Vec<u64>, usize) {
    let input_width = stride;
    let input_height = nodes.len() / input_width;
    let output_width = input_width * 5;
    let output_height = input_height * 5;
    let nodes = &nodes;
    (
        (0..output_height)
            .flat_map(|y| {
                (0..output_width).map(move |x| {
                    let input_grid_x = x % input_width;
                    let input_grid_y = y % input_height;
                    let repetition_x = (x / input_width) as u64;
                    let repetition_y = (y / input_height) as u64;
                    let mut value =
                        nodes[coordinate_to_index((input_grid_x, input_grid_y), input_width)]; // start with the associated value in the input grid
                    (0..repetition_x + repetition_y).for_each(|_| {
                        value %= 9;
                        value += 1;
                    }); // mod 9 + 1 will give us the result of wrapping back around to 1 due to mod 9 being 0.
                    value
                })
            })
            .collect(),
        output_width,
    )
}

/// Runs the A* algorithm over the grid, fixes the start at top-left and goal at bottom-right. Uses the
/// manhattan distance as the heuristic function.
fn find_shortest_path_cost(nodes: &Vec<u64>, stride: usize) -> u64 {
    // indices of the nodes pending exploration.
    // (0, 0) is the only node initially in the open set.
    let mut open_set = Vec::<usize>::with_capacity(nodes.len());
    open_set.push(0);

    // for a given node idx n, has the cheapest path score from the start to this node that is
    // currently known.
    let mut g_score = repeat(u64::MAX).take(nodes.len()).collect::<Vec<_>>();
    g_score[0] = 0;

    // for a given node idx n, f_score is the current best guess as to how short a path from
    // start to goal (through n) would be. Uses the g_score for the part already explored as we
    // know shortest here. Uses heuristic to estimate the distance from n to goal.
    let mut f_score = repeat(u64::MAX).take(nodes.len()).collect::<Vec<_>>();
    let last_index = nodes.len() - 1;
    let last_coordinate = index_to_coordinate(last_index, stride);
    f_score[0] = (last_coordinate.0 + last_coordinate.1) as u64;

    // just lets us handle neighbouring nodes better.
    // contains neighbouring coordinates and index
    let mut neighbours = Vec::<((usize, usize), usize)>::with_capacity(4);

    while !open_set.is_empty() {
        // find the node in the open set that has the lowest f_score.
        open_set.sort_by_key(|idx| Reverse(f_score[*idx]));
        let current = open_set.pop().unwrap();

        // if current is equal to the goal then we can take the f_score and return it.
        if current == last_index {
            return g_score[last_index];
        }

        // determine neighbouring coordinates/indices.
        let current_coordinate = index_to_coordinate(current, stride);

        if current_coordinate.0 > 0 {
            let neighbour_coord = (current_coordinate.0 - 1, current_coordinate.1);
            neighbours.push((
                neighbour_coord,
                coordinate_to_index(neighbour_coord, stride),
            ));
        }
        if current_coordinate.0 < last_coordinate.0 {
            let neighbour_coord = (current_coordinate.0 + 1, current_coordinate.1);
            neighbours.push((
                neighbour_coord,
                coordinate_to_index(neighbour_coord, stride),
            ))
        }
        if current_coordinate.1 > 0 {
            let neighbour_coord = (current_coordinate.0, current_coordinate.1 - 1);
            neighbours.push((
                neighbour_coord,
                coordinate_to_index(neighbour_coord, stride),
            ));
        }
        if current_coordinate.1 < last_coordinate.1 {
            let neighbour_coord = (current_coordinate.0, current_coordinate.1 + 1);
            neighbours.push((
                neighbour_coord,
                coordinate_to_index(neighbour_coord, stride),
            ));
        }

        // for each neighbour update the scores and add it to the open set if needed.
        neighbours
            .iter()
            .for_each(|(neighbour_coord, neighbour_idx)| {
                let (neighbour_coord, neighbour_idx) = (*neighbour_coord, *neighbour_idx);
                let tentative_score = g_score[current] + nodes[neighbour_idx]; // the weights are the entries in the nodes list.
                if tentative_score < g_score[neighbour_idx] {
                    // path to neighbour is better than the previous one.
                    g_score[neighbour_idx] = tentative_score;
                    f_score[neighbour_idx] = tentative_score
                        + calculate_manhattan_distance(last_coordinate, neighbour_coord) as u64;
                    if !open_set.contains(&neighbour_idx) {
                        open_set.push(neighbour_idx);
                    }
                }
            });

        // clear neighbours for next time.
        neighbours.clear();
    }

    u64::MAX
}

/// Converts the given index into a 2D coordinate by using the provided stride/row length.
fn index_to_coordinate(idx: usize, stride: usize) -> (usize, usize) {
    let y = idx / stride;
    let x = idx % stride;
    (x, y)
}

/// Converts the given 2D coordinate into an index by using the provided stride/row length;
fn coordinate_to_index(coord: (usize, usize), stride: usize) -> usize {
    let (x, y) = coord;
    y * stride + x
}

/// Calculates the manhattan distance between two coordinates. This is the sum of the absolute
/// differences between their components.
fn calculate_manhattan_distance(coord_1: (usize, usize), coord_2: (usize, usize)) -> usize {
    let abs_diff_x = if coord_1.0 > coord_2.0 {
        coord_1.0 - coord_2.0
    } else {
        coord_2.0 - coord_1.0
    };
    let abs_diff_y = if coord_1.1 > coord_2.1 {
        coord_1.1 - coord_2.1
    } else {
        coord_2.1 - coord_1.1
    };
    abs_diff_x + abs_diff_y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_parse_input_invalid_line_len() {
        let _ = parse_input(
            r#"
                1234
                678
                9012
            "#,
        );
    }

    #[test]
    #[should_panic]
    fn test_parse_input_invalid_digit() {
        let _ = parse_input(
            r#"
                1234
                5678
                9o12
            "#,
        );
    }

    #[test]
    fn test_parse_input_valid() {
        let (cells, stride) = parse_input(
            r#"
                1234
                5678
                9012
            "#,
        );
        assert_eq!(stride, 4);
        assert_eq!(cells, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);
    }

    #[test]
    fn test_index_to_coordinate() {
        assert_eq!(index_to_coordinate(0, 10), (0, 0));
        assert_eq!(index_to_coordinate(7, 4), (3, 1));
        assert_eq!(index_to_coordinate(11, 4), (3, 2));
    }

    #[test]
    fn test_coordinate_to_index() {
        assert_eq!(coordinate_to_index((0, 0), 10), 0);
        assert_eq!(coordinate_to_index((7, 3), 10), 37);
        assert_eq!(coordinate_to_index((3, 3), 4), 15);
    }

    #[test]
    fn test_manhattan_distance() {
        assert_eq!(calculate_manhattan_distance((0, 0), (3, 5)), 8);
        assert_eq!(calculate_manhattan_distance((7, 3), (11, 8)), 9);
    }

    #[test]
    fn test_find_shortest_path_cost() {
        const INPUT: &str = r#"
        1163751742
        1381373672
        2136511328
        3694931569
        7463417111
        1319128137
        1359912421
        3125421639
        1293138521
        2311944581
        "#;
        const EXPECTED: u64 = 40;
        let (nodes, stride) = parse_input(INPUT);
        assert_eq!(find_shortest_path_cost(&nodes, stride), EXPECTED);
    }
}
