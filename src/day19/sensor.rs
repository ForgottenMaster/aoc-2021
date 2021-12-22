use {
    super::rotations::ROTATIONS,
    crate::common::math::Matrix,
    std::{convert::Infallible, str::FromStr},
};

/// Represents a single Sensor in the puzzle which can detect a certain number of points
/// from whatever its rotation happens to be. The points of a sensor will be stored for each
/// rotation. Additionally, the Sensor will store the distances between pairs of points.
pub struct Sensor<'a> {
    points: [(&'a Matrix<i64, 3, 3>, Vec<(i64, i64, i64)>); 24],
    distances: Vec<((i64, i64, i64), (i64, i64, i64), i64)>,
}

/// Calculates the manhattan distance between two points.
fn manhattan_distance(p1: &(i64, i64, i64), p2: &(i64, i64, i64)) -> i64 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs() + (p1.2 - p2.2).abs()
}

/// Applies the rotation at the given index to the list of points
/// given and returns the pairing of reference to the rotation, and the
/// new vector.
fn transform_positions(
    points: &[(i64, i64, i64)],
    idx: usize,
) -> (&'static Matrix<i64, 3, 3>, Vec<(i64, i64, i64)>) {
    (
        &ROTATIONS[idx],
        points.iter().map(|point| &ROTATIONS[idx] * point).collect(),
    )
}

impl FromStr for Sensor<'_> {
    type Err = Infallible; // just fail on error.

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let positions = string
            .trim()
            .lines()
            .skip(1)
            .map(|line| {
                let mut splits = line.trim().split(",");
                (
                    splits.next().unwrap().parse::<i64>().unwrap(),
                    splits.next().unwrap().parse::<i64>().unwrap(),
                    splits.next().unwrap().parse::<i64>().unwrap(),
                )
            })
            .collect::<Vec<_>>();
        let points = [
            transform_positions(&positions, 0),
            transform_positions(&positions, 1),
            transform_positions(&positions, 2),
            transform_positions(&positions, 3),
            transform_positions(&positions, 4),
            transform_positions(&positions, 5),
            transform_positions(&positions, 6),
            transform_positions(&positions, 7),
            transform_positions(&positions, 8),
            transform_positions(&positions, 9),
            transform_positions(&positions, 10),
            transform_positions(&positions, 11),
            transform_positions(&positions, 12),
            transform_positions(&positions, 13),
            transform_positions(&positions, 14),
            transform_positions(&positions, 15),
            transform_positions(&positions, 16),
            transform_positions(&positions, 17),
            transform_positions(&positions, 18),
            transform_positions(&positions, 19),
            transform_positions(&positions, 20),
            transform_positions(&positions, 21),
            transform_positions(&positions, 22),
            transform_positions(&positions, 23),
        ];
        let distances = positions
            .iter()
            .enumerate()
            .flat_map(|(idx, p1)| {
                positions
                    .iter()
                    .skip(idx + 1)
                    .map(move |p2| (*p1, *p2, manhattan_distance(p1, p2)))
            })
            .collect();
        Ok(Self { points, distances })
    }
}
