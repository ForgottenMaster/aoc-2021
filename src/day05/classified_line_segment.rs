use super::{delta::Delta, line_segment::LineSegment, point::Point};

/// An enumeration that allows a line segment to be classified for later interpretation.
/// In part 1 this classification is just horizontal or vertical.
pub enum ClassifiedLineSegment {
    Horizontal(LineSegment),
    Vertical(LineSegment),
    Diagonal(LineSegment),
}

impl From<LineSegment> for ClassifiedLineSegment {
    fn from(value: LineSegment) -> Self {
        let delta: Delta = (&value).into();
        let delta: (i64, i64) = (&delta).into();
        match delta {
            (_, 0) => ClassifiedLineSegment::Horizontal(value),
            (0, _) => ClassifiedLineSegment::Vertical(value),
            _ => ClassifiedLineSegment::Diagonal(value),
        }
    }
}

impl From<&ClassifiedLineSegment> for (Point, Point) {
    fn from(value: &ClassifiedLineSegment) -> Self {
        match value {
            ClassifiedLineSegment::Horizontal(value) => value.into(),
            ClassifiedLineSegment::Vertical(value) => value.into(),
            ClassifiedLineSegment::Diagonal(value) => value.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classified_line_segment_from_line_segment_horizontal() {
        let line_segment = "0,0 -> 10,0".parse::<LineSegment>().unwrap();
        let line_segment: ClassifiedLineSegment = line_segment.into();
        assert!(match line_segment {
            ClassifiedLineSegment::Horizontal(_) => true,
            _ => false,
        });
    }

    #[test]
    fn test_classified_line_segment_from_line_segment_vertical() {
        let line_segment = "0,0 -> 0,10".parse::<LineSegment>().unwrap();
        let line_segment: ClassifiedLineSegment = line_segment.into();
        assert!(match line_segment {
            ClassifiedLineSegment::Vertical(_) => true,
            _ => false,
        });
    }

    #[test]
    fn test_classified_line_segment_from_line_segment_diagonal() {
        let line_segment = "0,0 -> 1,1".parse::<LineSegment>().unwrap();
        let line_segment: ClassifiedLineSegment = line_segment.into();
        assert!(match line_segment {
            ClassifiedLineSegment::Diagonal(_) => true,
            _ => false,
        });
    }
}
