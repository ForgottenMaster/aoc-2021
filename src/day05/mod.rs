mod classified_line_segment;
mod classified_line_segment_iter;
mod delta;
mod line_segment;
mod parse_line_error;
mod parse_point_error;
mod point;

use {
    classified_line_segment::ClassifiedLineSegment,
    classified_line_segment_iter::ClassifiedLineSegmentIter,
    line_segment::LineSegment,
    std::{
        collections::HashMap,
        fmt::Display,
        fs::File,
        io::{BufRead, BufReader},
    },
};

#[derive(Debug, PartialEq)]
pub enum ExecutionError {}

pub fn run() -> (impl Display, impl Display) {
    let file = File::open("input/day05.txt").expect("Could not open file.");
    let reader = BufReader::new(file);
    let line_segments = load_line_segments(reader);
    let part_1 = count_intersections(&get_point_counts(line_segments.iter().filter(
        |line| match line {
            ClassifiedLineSegment::Horizontal(_) | ClassifiedLineSegment::Vertical(_) => true,
            _ => false,
        },
    )));
    let part_2 = count_intersections(&get_point_counts(line_segments.iter()));
    (part_1, part_2)
}

/// Loads the valid line segments from the given BufRead instance
fn load_line_segments(reader: impl BufRead) -> Vec<ClassifiedLineSegment> {
    reader
        .lines()
        .filter_map(|line| {
            let line_segment = line.ok()?.trim().parse::<LineSegment>().ok()?;
            let line_segment = ClassifiedLineSegment::try_from(line_segment).ok()?;
            Some(line_segment)
        })
        .collect()
}

/// Creates a HashMap of point to count from a set of classified lines.
fn get_point_counts<'a>(
    segments: impl Iterator<Item = &'a ClassifiedLineSegment>,
) -> HashMap<(u32, u32), u32> {
    let mut hm = HashMap::new();
    segments.for_each(|line| {
        ClassifiedLineSegmentIter::new(line).for_each(|point| {
            *hm.entry((&point).into()).or_insert(0) += 1;
        });
    });
    hm
}

/// Gets the number of intersection points.
fn count_intersections(counts: &HashMap<(u32, u32), u32>) -> usize {
    counts.into_iter().filter(|(_, count)| **count >= 2).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        const INPUT: &[u8] = r#"
        0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2
        "#
        .as_bytes();
        const EXPECTED: usize = 5;
        let segments = load_line_segments(INPUT);
        let counts = get_point_counts(segments.iter().filter(|line| match line {
            ClassifiedLineSegment::Horizontal(_) | ClassifiedLineSegment::Vertical(_) => true,
            _ => false,
        }));
        let intersections = count_intersections(&counts);
        assert_eq!(intersections, EXPECTED);
    }

    #[test]
    fn test_part_2_example() {
        const INPUT: &[u8] = r#"
        0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2
        "#
        .as_bytes();
        const EXPECTED: usize = 12;
        let segments = load_line_segments(INPUT);
        let counts = get_point_counts(segments.iter());
        let intersections = count_intersections(&counts);
        assert_eq!(intersections, EXPECTED);
    }
}
