/// Enumeration that exposes the bitmasks that we use to identify
/// a specific wire/connection.
#[derive(Debug, PartialEq)]
pub enum SegmentLabel {
    A = 1 << 0,
    B = 1 << 1,
    C = 1 << 2,
    D = 1 << 3,
    E = 1 << 4,
    F = 1 << 5,
    G = 1 << 6,
}

/// We could use TryFrom but we're taking the stance that if the input is wrong
/// then we just panic the program instead. In a real program we'd want to handle
/// with proper error types, etc.
impl From<char> for SegmentLabel {
    fn from(value: char) -> Self {
        match value {
            'a' => Self::A,
            'b' => Self::B,
            'c' => Self::C,
            'd' => Self::D,
            'e' => Self::E,
            'f' => Self::F,
            'g' => Self::G,
            _ => panic!("Character {} encountered which is invalid.", value),
        }
    }
}

/// It's only safe to convert a number that matches exactly one of the SegmentLabels.
/// The program will panic if not, but this is fine as it indicates a problem with the input.
impl From<u8> for SegmentLabel {
    fn from(value: u8) -> Self {
        if value == Self::A as u8 {
            Self::A
        } else if value == Self::B as u8 {
            Self::B
        } else if value == Self::C as u8 {
            Self::C
        } else if value == Self::D as u8 {
            Self::D
        } else if value == Self::E as u8 {
            Self::E
        } else if value == Self::F as u8 {
            Self::F
        } else if value == Self::G as u8 {
            Self::G
        } else {
            panic!("Value {} couldn't be converted to a SegmentLabel", value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segment_label_from_char_valid() {
        assert_eq!(SegmentLabel::from('a'), SegmentLabel::A);
        assert_eq!(SegmentLabel::from('b'), SegmentLabel::B);
        assert_eq!(SegmentLabel::from('c'), SegmentLabel::C);
        assert_eq!(SegmentLabel::from('d'), SegmentLabel::D);
        assert_eq!(SegmentLabel::from('e'), SegmentLabel::E);
        assert_eq!(SegmentLabel::from('f'), SegmentLabel::F);
        assert_eq!(SegmentLabel::from('g'), SegmentLabel::G);
    }

    #[test]
    #[should_panic]
    fn test_segment_label_from_char_invalid() {
        SegmentLabel::from('z');
    }

    #[test]
    fn test_segment_label_from_number_valid() {
        let number = 0b00100000;
        let label: SegmentLabel = number.into();
        assert_eq!(label, SegmentLabel::F);
    }

    #[test]
    #[should_panic]
    fn test_segment_label_from_number_invalid() {
        let number = 0b00100100;
        let _label: SegmentLabel = number.into();
    }
}
