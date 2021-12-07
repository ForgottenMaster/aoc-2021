use super::{delta::Delta, line_segment::LineSegment, point::Point};

/// An enumeration that allows a line segment to be classified for later interpretation.
/// In part 1 this classification is just horizontal or vertical.
pub enum ClassifiedLineSegment {
    Horizontal(LineSegment),
    Vertical(LineSegment),
}

impl TryFrom<LineSegment> for ClassifiedLineSegment {
    type Error = (); // not really an error to fail to classify, just means we don't care about it

    fn try_from(value: LineSegment) -> Result<Self, Self::Error> {
        let delta: Delta = (&value).into();
        let delta: (i64, i64) = (&delta).into();
        match delta {
            (_, 0) => Ok(ClassifiedLineSegment::Horizontal(value)),
            (0, _) => Ok(ClassifiedLineSegment::Vertical(value)),
            _ => Err(()),
        }
    }
}

impl From<&ClassifiedLineSegment> for (Point, Point) {
    fn from(value: &ClassifiedLineSegment) -> Self {
        match value {
            ClassifiedLineSegment::Horizontal(value) => value.into(),
            ClassifiedLineSegment::Vertical(value) => value.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classified_line_segment_from_line_segment_horizontal() {
        let line_segment = "0,0 -> 10,0".parse::<LineSegment>().unwrap();
        let line_segment: ClassifiedLineSegment = line_segment.try_into().unwrap();
        assert!(match line_segment {
            ClassifiedLineSegment::Horizontal(_) => true,
            _ => false,
        });
    }

    #[test]
    fn test_classified_line_segment_from_line_segment_vertical() {
        let line_segment = "0,0 -> 0,10".parse::<LineSegment>().unwrap();
        let line_segment: ClassifiedLineSegment = line_segment.try_into().unwrap();
        assert!(match line_segment {
            ClassifiedLineSegment::Vertical(_) => true,
            _ => false,
        });
    }

    #[test]
    fn test_classified_line_segment_from_line_segment_failure() {
        let line_segment = "0,0 -> 2,2".parse::<LineSegment>().unwrap();
        let line_segment: Result<ClassifiedLineSegment, ()> = line_segment.try_into();
        assert!(line_segment.is_err());
    }
}
