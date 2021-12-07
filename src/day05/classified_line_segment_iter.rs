use super::{classified_line_segment::ClassifiedLineSegment, delta::Delta, point::Point};

/// Struct which will produce points along the length of a classified line segment.
/// This is fairly easy to do because we know that the line is classified as either
/// a horizontal or vertical line segment. At the end of the day though it's still just
/// adding a delta to a point to get the next one.
pub struct ClassifiedLineSegmentIter {
    current: Point,
    end: Point,
    delta: Delta,
    exhausted: bool,
}

impl ClassifiedLineSegmentIter {
    pub fn new(from: &ClassifiedLineSegment) -> Self {
        let (current, end): (Point, Point) = from.into();
        let delta = &end - &current;
        let len = delta.len();
        let delta = delta / len;

        Self {
            current,
            end,
            delta,
            exhausted: false,
        }
    }
}

impl Iterator for ClassifiedLineSegmentIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.exhausted {
            None
        } else {
            let current = self.current.clone();
            if self.current == self.end {
                self.exhausted = true;
            } else {
                self.current += &self.delta;
            }
            Some(current)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::line_segment::LineSegment;
    use super::*;

    #[test]
    fn test_classified_line_segment_iter_horizontal() {
        let line_segment = "14,20 -> 16,20".parse::<LineSegment>().unwrap();
        let line_segment: Result<ClassifiedLineSegment, ()> = line_segment.try_into();
        let line_segment = ClassifiedLineSegmentIter::new(&line_segment.unwrap());
        assert_eq!(
            line_segment.collect::<Vec<_>>(),
            vec![
                "14,20".parse::<Point>().unwrap(),
                "15,20".parse::<Point>().unwrap(),
                "16,20".parse::<Point>().unwrap()
            ]
        );
    }

    #[test]
    fn test_classified_line_segment_iter_horizontal_backward() {
        let line_segment = "16,20 -> 14,20".parse::<LineSegment>().unwrap();
        let line_segment: Result<ClassifiedLineSegment, ()> = line_segment.try_into();
        let line_segment = ClassifiedLineSegmentIter::new(&line_segment.unwrap());
        assert_eq!(
            line_segment.collect::<Vec<_>>(),
            vec![
                "16,20".parse::<Point>().unwrap(),
                "15,20".parse::<Point>().unwrap(),
                "14,20".parse::<Point>().unwrap()
            ]
        );
    }

    #[test]
    fn test_classified_line_segment_iter_vertical() {
        let line_segment = "16,10 -> 16,13".parse::<LineSegment>().unwrap();
        let line_segment: Result<ClassifiedLineSegment, ()> = line_segment.try_into();
        let line_segment = ClassifiedLineSegmentIter::new(&line_segment.unwrap());
        assert_eq!(
            line_segment.collect::<Vec<_>>(),
            vec![
                "16,10".parse::<Point>().unwrap(),
                "16,11".parse::<Point>().unwrap(),
                "16,12".parse::<Point>().unwrap(),
                "16,13".parse::<Point>().unwrap()
            ]
        );
    }

    #[test]
    fn test_classified_line_segment_iter_vertical_backward() {
        let line_segment = "16,13 -> 16,10".parse::<LineSegment>().unwrap();
        let line_segment: Result<ClassifiedLineSegment, ()> = line_segment.try_into();
        let line_segment = ClassifiedLineSegmentIter::new(&line_segment.unwrap());
        assert_eq!(
            line_segment.collect::<Vec<_>>(),
            vec![
                "16,13".parse::<Point>().unwrap(),
                "16,12".parse::<Point>().unwrap(),
                "16,11".parse::<Point>().unwrap(),
                "16,10".parse::<Point>().unwrap()
            ]
        );
    }
}
