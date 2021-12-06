use {super::parse_point_error::ParsePointError, std::str::FromStr};

/// structure that holds an x and y coordinate
/// representing an individual point.
#[derive(Debug, PartialEq)]
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
}
