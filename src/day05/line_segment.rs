use {
    super::{parse_line_error::ParseLineError, point::Point},
    std::str::FromStr,
};

/// Struct holding a pair of points indicating the two end points of a line
#[derive(Debug, PartialEq)]
pub struct LineSegment(Point, Point);

impl FromStr for LineSegment {
    type Err = ParseLineError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let string = string.trim();
        let mut splits = string
            .split("->")
            .map(|point| point.trim().parse::<Point>());
        if let Some(p1) = splits.next() {
            if let Some(p2) = splits.next() {
                if let None = splits.next() {
                    Ok(Self(p1?, p2?))
                } else {
                    Err(ParseLineError::TooManyParts)
                }
            } else {
                Err(ParseLineError::NotEnoughParts)
            }
        } else {
            Err(ParseLineError::NotEnoughParts)
        }
    }
}

impl From<&LineSegment> for (Point, Point) {
    fn from(value: &LineSegment) -> Self {
        (value.0.clone(), value.1.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::super::delta::Delta;
    use super::*;

    #[test]
    fn test_line_segment_from_str_parse_point_error() {
        assert!(match "2i -> 29".parse::<LineSegment>() {
            Err(ParseLineError::ParsePointError(_)) => true,
            _ => false,
        });
    }

    #[test]
    fn test_line_segment_from_str_not_enough_parts() {
        assert!(match "21,82".parse::<LineSegment>() {
            Err(ParseLineError::NotEnoughParts) => true,
            _ => false,
        });
    }

    #[test]
    fn test_line_segment_from_str_too_many_parts() {
        assert!(match "21,82 -> 24,17 -> 74,19".parse::<LineSegment>() {
            Err(ParseLineError::TooManyParts) => true,
            _ => false,
        });
    }

    #[test]
    fn test_line_segment_from_str_success() {
        assert_eq!(
            "21,82 -> 24,17".parse::<LineSegment>().unwrap(),
            LineSegment("21,82".parse().unwrap(), "24,17".parse().unwrap())
        );
    }

    #[test]
    fn test_line_segment_delta() {
        let line_segment = "25,13 -> 92,3".parse::<LineSegment>().unwrap();
        let point_1 = "25,13".parse::<Point>().unwrap();
        let point_2 = "92,3".parse::<Point>().unwrap();
        let expected = &point_2 - &point_1;
        let calculated: Delta = (&line_segment).into();
        assert_eq!(expected, calculated);
    }
}
