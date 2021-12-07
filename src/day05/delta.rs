use {
    super::{line_segment::LineSegment, point::Point},
    std::ops::Div,
};

/// Struct representing the signed delta between two points.
/// Since the points contain u32 coordinates, we represent the delta with i64 as it's large
/// enough to contain the positive range of u32's
#[derive(Debug, PartialEq)]
pub struct Delta(i64, i64);

impl Delta {
    pub fn len(&self) -> i64 {
        ((self.0.pow(2) + self.1.pow(2)) as f64).sqrt() as i64
    }
}

impl Div<i64> for Delta {
    type Output = Self;

    fn div(self, rhs: i64) -> Self {
        Self(self.0 / rhs, self.1 / rhs)
    }
}

impl From<&LineSegment> for Delta {
    fn from(value: &LineSegment) -> Self {
        let (p1, p2): (Point, Point) = value.into();
        &p2 - &p1
    }
}

impl From<&Delta> for (i64, i64) {
    fn from(value: &Delta) -> Self {
        (value.0, value.1)
    }
}

impl From<(i64, i64)> for Delta {
    fn from(value: (i64, i64)) -> Self {
        Self(value.0, value.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delta_from_to_point() {
        let point_1 = "21,45".parse::<Point>().unwrap();
        let point_2 = "93,21".parse::<Point>().unwrap();
        let expected = Delta(72, -24);
        let calculated = &point_2 - &point_1;
        assert_eq!(expected, calculated);
    }

    #[test]
    fn test_delta_to_tuple() {
        let delta = Delta(17, 34);
        let expected = (17, 34);
        let calculated: (i64, i64) = (&delta).into();
        assert_eq!(expected, calculated);
    }
}
