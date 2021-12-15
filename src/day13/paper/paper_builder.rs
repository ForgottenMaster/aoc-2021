use {super::Paper, std::iter::repeat};

/// Struct which accumulates the dots on a piece of transparent paper and makes sure
/// that the grid remains rectangular. Can be consumed after adding coordinates to produce
/// the final sheet of Paper.
#[derive(Debug, Default, PartialEq)]
pub struct PaperBuilder {
    storage: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl PaperBuilder {
    /// Expands the internal storage in a rectangular fashion and plots a point down
    /// at the given coordinate.
    pub fn place_dot(&mut self, x: usize, y: usize) {
        if self.width <= x {
            self.expand_sheet_horizontally(x - self.width + 1);
        }
        if self.height <= y {
            self.expand_sheet_vertically(y - self.height + 1);
        }
        self.storage[y][x] = true;
    }

    /// Consumes the builder and attempts to produce a sheet of Paper with the dots plotted
    /// as defined. The only possible way to fail the build is is place_dot was never called
    /// first. Since this is an expected error we just return Option<Paper> rather than Result<Paper>.
    pub fn build(self) -> Option<Paper> {
        Paper::new(self.storage).ok()
    }

    /// Expands the sheet horizontally by the given amount. This just expands
    /// each line of the paper by this amount of false entries, and records the
    /// expanded width.
    fn expand_sheet_horizontally(&mut self, amount: usize) {
        self.storage.iter_mut().for_each(|line| {
            line.extend(repeat(false).take(amount));
        });
        self.width += amount;
    }

    /// Expands the sheet vertically by the given amount. This just adds new lines
    /// of falses to the vector of the current width length and records the new height.
    fn expand_sheet_vertically(&mut self, amount: usize) {
        let empty_line = repeat(false).take(self.width).collect::<Vec<_>>();
        self.storage.extend(repeat(empty_line).take(amount));
        self.height += amount;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_sheet_vertically_zero() {
        let mut builder = PaperBuilder::default();
        builder.expand_sheet_vertically(0);
        assert_eq!(
            builder,
            PaperBuilder {
                storage: vec![],
                width: 0,
                height: 0
            }
        );
    }

    #[test]
    fn test_expand_sheet_vertically_non_zero() {
        let mut builder = PaperBuilder::default();
        builder.expand_sheet_vertically(2);
        assert_eq!(
            builder,
            PaperBuilder {
                storage: vec![vec![], vec![]],
                width: 0,
                height: 2
            }
        );
    }

    #[test]
    fn test_expand_sheet_horizontally_zero() {
        let mut builder = PaperBuilder::default();
        builder.expand_sheet_horizontally(0);
        assert_eq!(
            builder,
            PaperBuilder {
                storage: vec![],
                width: 0,
                height: 0
            }
        );
    }

    #[test]
    fn test_expand_sheet_horizontally_non_zero() {
        let mut builder = PaperBuilder::default();
        builder.expand_sheet_horizontally(2);
        assert_eq!(
            builder,
            PaperBuilder {
                storage: vec![],
                width: 2,
                height: 0
            }
        );
    }

    #[test]
    fn test_expand_sheet_horizontally_then_vertically() {
        let mut builder = PaperBuilder::default();
        builder.expand_sheet_horizontally(2);
        builder.expand_sheet_vertically(2);
        assert_eq!(
            builder,
            PaperBuilder {
                storage: vec![vec![false, false], vec![false, false]],
                width: 2,
                height: 2
            }
        );
    }

    #[test]
    fn test_expand_sheet_vertically_then_horizontally() {
        let mut builder = PaperBuilder::default();
        builder.expand_sheet_vertically(2);
        builder.expand_sheet_horizontally(2);
        assert_eq!(
            builder,
            PaperBuilder {
                storage: vec![vec![false, false], vec![false, false]],
                width: 2,
                height: 2
            }
        );
    }

    #[test]
    fn test_place_dot_origin() {
        let mut builder = PaperBuilder::default();
        builder.place_dot(0, 0);
        assert_eq!(
            builder,
            PaperBuilder {
                storage: vec![vec![true]],
                width: 1,
                height: 1
            }
        );
    }

    #[test]
    fn test_place_dot_make_expand_horizontally() {
        let mut builder = PaperBuilder::default();
        builder.place_dot(3, 0);
        assert_eq!(
            builder,
            PaperBuilder {
                storage: vec![vec![false, false, false, true]],
                width: 4,
                height: 1
            }
        );
    }

    #[test]
    fn test_place_dot_make_expand_vertically() {
        let mut builder = PaperBuilder::default();
        builder.place_dot(0, 3);
        assert_eq!(
            builder,
            PaperBuilder {
                storage: vec![vec![false], vec![false], vec![false], vec![true]],
                width: 1,
                height: 4
            }
        );
    }

    #[test]
    fn test_place_multi_dots_expand_to_furthest() {
        let mut builder = PaperBuilder::default();
        builder.place_dot(1, 1);
        builder.place_dot(3, 2);
        assert_eq!(
            builder,
            PaperBuilder {
                storage: vec![
                    vec![false, false, false, false],
                    vec![false, true, false, false],
                    vec![false, false, false, true]
                ],
                width: 4,
                height: 3
            }
        );
    }

    #[test]
    fn test_place_multi_dots_largest_first() {
        let mut builder = PaperBuilder::default();
        builder.place_dot(3, 2);
        builder.place_dot(1, 1);
        assert_eq!(
            builder,
            PaperBuilder {
                storage: vec![
                    vec![false, false, false, false],
                    vec![false, true, false, false],
                    vec![false, false, false, true]
                ],
                width: 4,
                height: 3
            }
        );
    }

    #[test]
    fn test_build_failure() {
        let builder = PaperBuilder::default();
        assert!(builder.build().is_none());
    }

    #[test]
    fn test_build_success() {
        let mut builder = PaperBuilder::default();
        builder.place_dot(1, 1);
        let expected = Paper::new(vec![vec![false, false], vec![false, true]]).unwrap();
        assert_eq!(builder.build().unwrap(), expected);
    }
}
