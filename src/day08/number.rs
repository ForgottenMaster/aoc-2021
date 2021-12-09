use {
    super::segment_label::SegmentLabel,
    std::ops::{BitAnd, BitXor},
};

/// Struct containing a representation of a number
/// that is formed from combining various segments together.
#[derive(Debug, PartialEq)]
pub struct Number(u8);

/// Same as with the SegmentLabel parsing, if we encounter
/// an error in the parsing, just panic. This will happen for us actually
/// as we'll be running through the characters of the string and converting
/// those to SegmentLabel. A Number then, is just a masking of the segments
/// that makes it up (but we don't know where those segment labels).
impl From<&str> for Number {
    fn from(value: &str) -> Self {
        Self(
            value
                .trim()
                .chars()
                .map(|c| SegmentLabel::from(c) as u8)
                .sum(),
        )
    }
}

/// It's safe to expose the inner number since it can't be wrapped back up
/// into a number again.
impl From<&Number> for u8 {
    fn from(value: &Number) -> Self {
        value.0
    }
}

impl From<Number> for u8 {
    fn from(value: Number) -> Self {
        value.0
    }
}

impl BitXor<Self> for &Number {
    type Output = Number;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Number(self.0 ^ rhs.0)
    }
}

impl BitXor<Self> for Number {
    type Output = Number;

    fn bitxor(self, rhs: Self) -> Self::Output {
        &self ^ &rhs
    }
}

impl BitAnd<Self> for &Number {
    type Output = Number;

    fn bitand(self, rhs: Self) -> Self::Output {
        Number(self.0 & rhs.0)
    }
}

impl BitAnd<Self> for Number {
    type Output = Number;

    fn bitand(self, rhs: Self) -> Self::Output {
        &self & &rhs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_from_str_valid() {
        const EXPECTED: u8 = SegmentLabel::A as u8 | SegmentLabel::D as u8;
        assert_eq!(Number::from("ad").0, EXPECTED);
        assert_eq!(Number::from("da").0, EXPECTED);
    }

    #[test]
    #[should_panic]
    fn test_number_from_str_invalid() {
        Number::from("adz");
    }

    #[test]
    fn test_number_bit_xor() {
        assert_eq!(Number::from("def") ^ Number::from("ef"), Number::from("d"));
    }

    #[test]
    fn test_number_bit_and() {
        assert_eq!(Number::from("def") & Number::from("dga"), Number::from("d"));
    }
}
