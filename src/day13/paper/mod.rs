mod build_paper_error;
mod paper_builder;

pub use paper_builder::PaperBuilder;
use {
    build_paper_error::BuildPaperError,
    std::{
        cmp::max,
        fmt,
        fmt::{Display, Formatter},
        iter::{once, repeat},
    },
}; // required as we use the Builder to construct the sheet of paper.

/// Struct that represents the transparent sheet of foldable paper.
/// Internally this stores a vector of vectors of booleans indicating if the
/// cell has a dot in it or not. Allows us to fold up the paper either vertically or horizontally.
#[derive(Debug, PartialEq)]
pub struct Paper {
    storage: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl Paper {
    /// This attempts to construct a new sheet of paper from the given
    /// 2D grid of boolean values. Will return None if the given grid isn't
    /// rectangular or if there are no entries in either direction.
    fn new(storage: Vec<Vec<bool>>) -> Result<Self, BuildPaperError> {
        let height = storage.len();
        if height == 0 {
            Err(BuildPaperError::InvalidHeight)
        } else {
            let width = storage[0].len();
            if width == 0 {
                Err(BuildPaperError::InvalidWidth)
            } else if storage.iter().skip(1).all(|line| line.len() == width) {
                Ok(Self {
                    storage,
                    width,
                    height,
                })
            } else {
                Err(BuildPaperError::NonEqualWidth)
            }
        }
    }

    /// Function that folds the sheet of paper UPWARDS around a given line. The line
    /// will not be included in the resulting folded paper and lines below it are flipped
    /// and merged onto lines above it.
    pub fn fold_vertically(&mut self, around: usize) {
        let bottom_section_height = self.height - 1 - around;
        let top_section_height = around;
        let max_section_height = max(bottom_section_height, top_section_height);
        let top_section_padding = max_section_height - top_section_height;
        let bottom_section_padding = max_section_height - bottom_section_height;

        // The paper is folded upwards so the top section remains ordered the same, but padding
        // lines are added before it to make both sections equal length for the zip.
        let padding_top = repeat(
            repeat(false)
                .take(self.width)
                .collect::<Vec<_>>()
                .into_iter(),
        )
        .take(top_section_padding);
        let selection_top = self
            .storage
            .iter()
            .map(|line| {
                line.into_iter()
                    .map(|elem| *elem)
                    .collect::<Vec<_>>()
                    .into_iter()
            })
            .take(top_section_height);
        let iter_top = padding_top.chain(selection_top);

        // The bottom section will be reversed, but padding still added before it.
        let padding_bottom = repeat(
            repeat(false)
                .take(self.width)
                .collect::<Vec<_>>()
                .into_iter(),
        )
        .take(bottom_section_padding);
        let selection_bottom = self
            .storage
            .iter()
            .rev()
            .map(|line| {
                line.into_iter()
                    .map(|elem| *elem)
                    .collect::<Vec<_>>()
                    .into_iter()
            })
            .take(bottom_section_height);
        let iter_bottom = padding_bottom.chain(selection_bottom);

        // Zip these together as they're the same length now, each entry becomes the OR operation on both input entries.
        self.storage = iter_top
            .zip(iter_bottom)
            .map(|(line_1_iter, line_2_iter)| {
                line_1_iter
                    .zip(line_2_iter)
                    .map(|(elem_1, elem_2)| elem_1 || elem_2)
                    .collect()
            })
            .collect();
        self.height = max_section_height;
    }

    /// Folds the paper to the left around a given point horizontally.
    pub fn fold_horizontally(&mut self, around: usize) {
        let right_section_width = self.width - 1 - around;
        let left_section_width = around;
        let max_section_width = max(left_section_width, right_section_width);
        let left_section_padding = max_section_width - left_section_width;
        let right_section_padding = max_section_width - right_section_width;

        self.storage = self
            .storage
            .iter()
            .map(|line| {
                let padding_left = repeat(false).take(left_section_padding);
                let selection_left = line.iter().map(|elem| *elem).take(left_section_width);
                let iter_left = padding_left.chain(selection_left);

                let padding_right = repeat(false).take(right_section_padding);
                let selection_right = line
                    .iter()
                    .rev()
                    .map(|elem| *elem)
                    .take(right_section_width);
                let iter_right = padding_right.chain(selection_right);

                iter_left
                    .zip(iter_right)
                    .map(|(elem_1, elem_2)| elem_1 || elem_2)
                    .collect()
            })
            .collect();
        self.width = max_section_width;
    }

    /// Counts the number of dots (true values) present in the grid
    pub fn count_dots(&self) -> usize {
        self.storage
            .iter()
            .map(|line| line.into_iter().filter(|elem| **elem).count())
            .sum()
    }
}

impl Display for Paper {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(
            &self
                .storage
                .iter()
                .flat_map(|line| {
                    line.into_iter()
                        .map(|has_dot| if *has_dot { '#' } else { '.' })
                        .chain(once('\n'))
                })
                .collect::<String>(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_paper_invalid_height() {
        assert_eq!(Paper::new(vec![]), Err(BuildPaperError::InvalidHeight));
    }

    #[test]
    fn test_new_paper_invalid_width() {
        assert_eq!(Paper::new(vec![vec![]]), Err(BuildPaperError::InvalidWidth));
    }

    #[test]
    fn test_new_paper_non_equal_width() {
        assert_eq!(
            Paper::new(vec![vec![true], vec![true, false]]),
            Err(BuildPaperError::NonEqualWidth)
        );
    }

    #[test]
    fn test_new_paper_success() {
        assert_eq!(
            Paper::new(vec![vec![true, false], vec![false, true]]),
            Ok(Paper {
                storage: vec![vec![true, false], vec![false, true]],
                width: 2,
                height: 2
            })
        );
    }

    #[test]
    fn test_vertical_fold_around_zero() {
        let mut paper = Paper::new(vec![
            vec![false, true, false],
            vec![true, false, true],
            vec![true, true, false],
        ])
        .unwrap();
        let expected = vec![vec![true, true, false], vec![true, false, true]];
        paper.fold_vertically(0);
        assert_eq!(paper.storage, expected);
        assert_eq!(paper.width, 3);
        assert_eq!(paper.height, 2);
    }

    #[test]
    fn test_vertical_fold_around_last() {
        let mut paper = Paper::new(vec![
            vec![false, true, false],
            vec![true, false, true],
            vec![true, true, false],
        ])
        .unwrap();
        let expected = vec![vec![false, true, false], vec![true, false, true]];
        paper.fold_vertically(2);
        assert_eq!(paper.storage, expected);
        assert_eq!(paper.width, 3);
        assert_eq!(paper.height, 2);
    }

    #[test]
    fn test_vertical_fold_middle() {
        let mut paper = Paper::new(vec![
            vec![true, false, true, false, false, true],
            vec![true, true, true, false, false, true],
            vec![false, false, true, false, true, false],
            vec![false, false, false, true, true, false],
            vec![false, true, true, false, false, false],
        ])
        .unwrap();
        let expected = vec![
            vec![true, true, true, false, false, true],
            vec![true, true, true, true, true, true],
        ];
        paper.fold_vertically(2);
        assert_eq!(paper.storage, expected);
        assert_eq!(paper.width, 6);
        assert_eq!(paper.height, 2);
    }

    #[test]
    fn test_vertical_fold_top_smaller() {
        let mut paper = Paper::new(vec![
            vec![true, false, true, false, false, true],
            vec![true, true, true, false, false, true],
            vec![false, false, true, false, true, false],
            vec![false, false, false, true, true, false],
            vec![false, true, true, false, false, false],
        ])
        .unwrap();
        let expected = vec![
            vec![false, true, true, false, false, false],
            vec![false, false, false, true, true, false],
            vec![true, false, true, false, true, true],
        ];
        paper.fold_vertically(1);
        assert_eq!(paper.storage, expected);
        assert_eq!(paper.width, 6);
        assert_eq!(paper.height, 3);
    }

    #[test]
    fn test_vertical_fold_bottom_smaller() {
        let mut paper = Paper::new(vec![
            vec![true, false, true, false, false, true],
            vec![true, true, true, false, false, true],
            vec![false, false, true, false, true, false],
            vec![false, false, false, true, true, false],
            vec![false, true, true, false, false, false],
        ])
        .unwrap();
        let expected = vec![
            vec![true, false, true, false, false, true],
            vec![true, true, true, false, false, true],
            vec![false, true, true, false, true, false],
        ];
        paper.fold_vertically(3);
        assert_eq!(paper.storage, expected);
        assert_eq!(paper.width, 6);
        assert_eq!(paper.height, 3);
    }

    #[test]
    fn test_horizontal_fold_around_left() {
        let mut paper = Paper::new(vec![
            vec![true, false, true, false, false, true],
            vec![true, true, true, false, false, true],
            vec![false, false, true, false, true, false],
            vec![false, false, false, true, true, false],
            vec![false, true, true, false, false, false],
        ])
        .unwrap();
        let expected = vec![
            vec![true, false, false, true, false],
            vec![true, false, false, true, true],
            vec![false, true, false, true, false],
            vec![false, true, true, false, false],
            vec![false, false, false, true, true],
        ];
        paper.fold_horizontally(0);
        assert_eq!(paper.storage, expected);
        assert_eq!(paper.width, 5);
        assert_eq!(paper.height, 5);
    }

    #[test]
    fn test_horizontal_fold_around_right() {
        let mut paper = Paper::new(vec![
            vec![true, false, true, false, false, true],
            vec![true, true, true, false, false, true],
            vec![false, false, true, false, true, false],
            vec![false, false, false, true, true, false],
            vec![false, true, true, false, false, false],
        ])
        .unwrap();
        let expected = vec![
            vec![true, false, true, false, false],
            vec![true, true, true, false, false],
            vec![false, false, true, false, true],
            vec![false, false, false, true, true],
            vec![false, true, true, false, false],
        ];
        paper.fold_horizontally(5);
        assert_eq!(paper.storage, expected);
        assert_eq!(paper.width, 5);
        assert_eq!(paper.height, 5);
    }

    #[test]
    fn test_horizontal_fold_around_middle() {
        let mut paper = Paper::new(vec![
            vec![true, false, true, false, false],
            vec![true, true, true, false, false],
            vec![false, false, true, false, true],
            vec![false, false, false, true, true],
            vec![false, true, true, false, false],
        ])
        .unwrap();
        let expected = vec![
            vec![true, false],
            vec![true, true],
            vec![true, false],
            vec![true, true],
            vec![false, true],
        ];
        paper.fold_horizontally(2);
        assert_eq!(paper.storage, expected);
        assert_eq!(paper.width, 2);
        assert_eq!(paper.height, 5);
    }

    #[test]
    fn test_horizontal_fold_larger_left_side() {
        let mut paper = Paper::new(vec![
            vec![true, false, true, false, false],
            vec![true, true, true, false, false],
            vec![false, false, true, false, true],
            vec![false, false, false, true, true],
            vec![false, true, true, false, false],
        ])
        .unwrap();
        let expected = vec![
            vec![true, false, true],
            vec![true, true, true],
            vec![false, false, true],
            vec![false, false, true],
            vec![false, true, true],
        ];
        paper.fold_horizontally(3);
        assert_eq!(paper.storage, expected);
        assert_eq!(paper.width, 3);
        assert_eq!(paper.height, 5);
    }

    #[test]
    fn test_horizontal_fold_larger_right_side() {
        let mut paper = Paper::new(vec![
            vec![true, false, true, false, false],
            vec![true, true, true, false, false],
            vec![false, false, true, false, true],
            vec![false, false, false, true, true],
            vec![false, true, true, false, false],
        ])
        .unwrap();
        let expected = vec![
            vec![false, false, true],
            vec![false, false, true],
            vec![true, false, true],
            vec![true, true, false],
            vec![false, false, true],
        ];
        paper.fold_horizontally(1);
        assert_eq!(paper.storage, expected);
        assert_eq!(paper.width, 3);
        assert_eq!(paper.height, 5);
    }

    #[test]
    fn test_count_dots() {
        let paper = Paper::new(vec![
            vec![true, false, true, false, false],
            vec![true, true, true, false, false],
            vec![false, false, true, false, true],
            vec![false, false, false, true, true],
            vec![false, true, true, false, false],
        ])
        .unwrap();
        assert_eq!(paper.count_dots(), 11);
    }

    #[test]
    fn test_paper_display() {
        let paper = Paper::new(vec![
            vec![true, false, true, false, false],
            vec![true, true, true, false, false],
            vec![false, false, true, false, true],
            vec![false, false, false, true, true],
            vec![false, true, true, false, false],
        ])
        .unwrap();
        const EXPECTED: &str = "#.#..\n###..\n..#.#\n...##\n.##..\n";
        assert_eq!(format!("{}", paper), EXPECTED);
    }
}
