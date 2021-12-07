use {
    super::{delta::Delta, parse_point_error::ParsePointError},
    std::{
        ops::{AddAssign, Sub},
        str::FromStr,
    },
};

/// structure that holds an x and y coordinate
/// representing an individual point.
#[derive(Clone, Debug, PartialEq)]
pub struct Point(u32, u32);

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let string = string.trim();
        let mut splits = string.split(",").map(|coord| coord.trim().parse::<u32>());
        if let Some(x) = splits.next() {
            if let Some(y) = splits.next() {
                if let None = splits.next() {
                    Ok(Self(x?, y?))
                } else {
                    Err(ParsePointError::TooManyParts)
                }
            } else {
                Err(ParsePointError::NotEnoughParts)
            }
        } else {
            Err(ParsePointError::NotEnoughParts)
        }
    }
}

impl AddAssign<&Delta> for Point {
    fn add_assign(&mut self, rhs: &Delta) {
        let (dx, dy): (i64, i64) = rhs.into();
        self.0 = (self.0 as i64 + dx) as u32;
        self.1 = (self.1 as i64 + dy) as u32;
    }
}

impl From<&Point> for (u32, u32) {
    fn from(value: &Point) -> Self {
        (value.0, value.1)
    }
}

impl Sub<&Point> for &Point {
    type Output = Delta;

    fn sub(self, rhs: &Point) -> Delta {
        let (p1x, p1y): (u32, u32) = self.into();
        let (p2x, p2y): (u32, u32) = rhs.into();
        let delta = (p1x as i64 - p2x as i64, p1y as i64 - p2y as i64);
        delta.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_from_str_not_enough_parts() {
        assert!(match "17".parse::<Point>() {
            Err(ParsePointError::NotEnoughParts) => true,
            _ => false,
        });
    }

    #[test]
    fn test_point_from_str_too_many_parts() {
        assert!(match "17, 21, 22".parse::<Point>() {
            Err(ParsePointError::TooManyParts) => true,
            _ => false,
        });
    }

    #[test]
    fn test_point_from_str_parse_int_error() {
        assert!(match "17, 2i".parse::<Point>() {
            Err(ParsePointError::ParseIntError(_)) => true,
            _ => false,
        });
    }

    #[test]
    fn test_point_from_str_success() {
        assert_eq!("17, 21".parse::<Point>().unwrap(), Point(17, 21));
    }

    #[test]
    fn test_point_to_tuple() {
        let point = "17,21".parse::<Point>().unwrap();
        let point: (u32, u32) = (&point).into();
        assert_eq!(point, (17, 21));
    }
}
