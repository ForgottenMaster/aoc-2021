use std::{cell::RefCell, convert::Infallible, ops::Add, rc::Rc, str::FromStr};

/// Represents the type of an element of a pair in Snailfish numbers.
/// An element can either be another pair of NumberType or can be
/// a regular number.
#[derive(Debug, PartialEq)]
pub enum NumberType {
    Pair(Rc<RefCell<NumberType>>, Rc<RefCell<NumberType>>), // need to box here because otherwise would be recursive/infinite size. We use Rc here as we'll be pushing pointers onto the stack later.
    Regular(u8),
}

impl NumberType {
    /// Attempts to decompose this NumberType as a pair, if successful returns
    /// cloned Rc instances. If unsuccessful returns None.
    pub fn decompose_pair(&self) -> Option<(Rc<RefCell<NumberType>>, Rc<RefCell<NumberType>>)> {
        match self {
            Self::Pair(p1, p2) => Some((p1.clone(), p2.clone())),
            _ => None,
        }
    }

    /// Attempts to extract the contained number from the NumberType, if it's of
    /// type Regular.
    pub fn extract_regular(&self) -> Option<u8> {
        match self {
            Self::Regular(val) => Some(*val),
            _ => None,
        }
    }

    /// Calculates the magnitude of the NumberType recursively.
    pub fn magnitude(&self) -> u64 {
        match self {
            Self::Regular(value) => *value as u64,
            Self::Pair(p1, p2) => p1.borrow().magnitude() * 3 + p2.borrow().magnitude() * 2,
        }
    }
}

impl Add for NumberType {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Pair(Rc::new(RefCell::new(self)), Rc::new(RefCell::new(rhs)))
    }
}

// Required as we want a deep clone.
impl Clone for NumberType {
    fn clone(&self) -> Self {
        match self {
            Self::Regular(val) => Self::Regular(*val),
            Self::Pair(p1, p2) => Self::Pair(
                Rc::new(RefCell::new((*p1.borrow()).clone())),
                Rc::new(RefCell::new((*p2.borrow()).clone())),
            ),
        }
    }
}

impl FromStr for NumberType {
    type Err = Infallible;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut stack = vec![]; // used to parse
        string.trim().chars().for_each(|c| match c {
            '[' | ',' | ' ' => return,
            ']' => {
                let second = stack.pop().unwrap();
                let first = stack.pop().unwrap();
                stack.push(Rc::new(RefCell::new(NumberType::Pair(first, second))));
            }
            _ => stack.push(Rc::new(RefCell::new(NumberType::Regular(
                c.to_digit(10).unwrap() as u8,
            )))),
        });
        let parsed = (*stack.pop().unwrap().borrow()).clone();
        if stack.len() != 0 {
            panic!("Invalid data format, parse stack isn't empty after parsing.")
        }
        Ok(parsed)
    }
}

/// Extracts the lowest literal down the right branch.
pub fn lowest_literal_down_right_branch(ptr: &Rc<RefCell<NumberType>>) -> Rc<RefCell<NumberType>> {
    match &*ptr.borrow() {
        NumberType::Regular(..) => Rc::clone(ptr),
        NumberType::Pair(_, p2) => lowest_literal_down_right_branch(p2),
    }
}

/// Extracts the lowest literal down the left branch.
pub fn lowest_literal_down_left_branch(ptr: &Rc<RefCell<NumberType>>) -> Rc<RefCell<NumberType>> {
    match &*ptr.borrow() {
        NumberType::Regular(..) => Rc::clone(ptr),
        NumberType::Pair(p1, _) => lowest_literal_down_left_branch(p1),
    }
}

/// Accepts two Rc<RefCell<NumberType>> references and determines if one is located somewhere
/// in the other one (or is the other one).
pub fn contains_number_type(
    haystack: &Rc<RefCell<NumberType>>,
    needle: &Rc<RefCell<NumberType>>,
) -> bool {
    if Rc::ptr_eq(needle, haystack) {
        true
    } else {
        match *haystack.borrow() {
            NumberType::Regular(..) => false,
            NumberType::Pair(ref p1, ref p2) => {
                Rc::ptr_eq(p1, needle)
                    || Rc::ptr_eq(p2, needle)
                    || contains_number_type(p1, needle)
                    || contains_number_type(p2, needle)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_type_add() {
        let number_1 = "[1, 2]".parse::<NumberType>().unwrap();
        let number_2 = "[[3, 4], 5]".parse::<NumberType>().unwrap();
        let expected = "[[1, 2], [[3, 4], 5]]".parse::<NumberType>().unwrap();
        let calculated = number_1 + number_2;
        assert_eq!(expected, calculated);
    }

    #[test]
    fn test_decompose_pair() {
        let number_1 = "[1, 2]".parse::<NumberType>().unwrap();
        let number_2 = "3".parse::<NumberType>().unwrap();
        assert_eq!(
            number_1.decompose_pair(),
            Some((
                Rc::new(RefCell::new("1".parse::<NumberType>().unwrap())),
                Rc::new(RefCell::new("2".parse::<NumberType>().unwrap()))
            ))
        );
        assert_eq!(number_2.decompose_pair(), None);
    }

    #[test]
    fn test_extract_regular() {
        let number_1 = "[1, 2]".parse::<NumberType>().unwrap();
        let number_2 = "3".parse::<NumberType>().unwrap();
        assert_eq!(number_1.extract_regular(), None);
        assert_eq!(number_2.extract_regular(), Some(3));
    }

    #[test]
    fn test_contains_number_type_needle_equal_haystack() {
        let needle = Rc::new(RefCell::new("7".parse::<NumberType>().unwrap()));
        let haystack = Rc::clone(&needle);
        assert!(contains_number_type(&haystack, &needle));
    }

    #[test]
    fn test_contains_number_type_haystack_literal() {
        let needle = Rc::new(RefCell::new("7".parse::<NumberType>().unwrap()));
        let haystack = Rc::new(RefCell::new("3".parse::<NumberType>().unwrap()));
        assert!(!contains_number_type(&haystack, &needle));
    }

    #[test]
    fn test_contains_number_type_haystack_pair_first_equal() {
        let needle = Rc::new(RefCell::new("7".parse::<NumberType>().unwrap()));
        let haystack = Rc::new(RefCell::new(NumberType::Pair(
            Rc::clone(&needle),
            Rc::new(RefCell::new("3".parse::<NumberType>().unwrap())),
        )));
        assert!(contains_number_type(&haystack, &needle));
    }

    #[test]
    fn test_contains_number_type_haystack_pair_second_equal() {
        let needle = Rc::new(RefCell::new("7".parse::<NumberType>().unwrap()));
        let haystack = Rc::new(RefCell::new(NumberType::Pair(
            Rc::new(RefCell::new("3".parse::<NumberType>().unwrap())),
            Rc::clone(&needle),
        )));
        assert!(contains_number_type(&haystack, &needle));
    }

    #[test]
    fn test_contains_number_type_haystack_pair_recurse_first() {
        let needle = Rc::new(RefCell::new("7".parse::<NumberType>().unwrap()));
        let haystack = Rc::new(RefCell::new(NumberType::Pair(
            Rc::new(RefCell::new(NumberType::Pair(
                Rc::clone(&needle),
                Rc::new(RefCell::new("5".parse::<NumberType>().unwrap())),
            ))),
            Rc::new(RefCell::new("3".parse::<NumberType>().unwrap())),
        )));
        assert!(contains_number_type(&haystack, &needle));
    }

    #[test]
    fn test_contains_number_type_haystack_pair_recurse_second() {
        let needle = Rc::new(RefCell::new("7".parse::<NumberType>().unwrap()));
        let haystack = Rc::new(RefCell::new(NumberType::Pair(
            Rc::new(RefCell::new("3".parse::<NumberType>().unwrap())),
            Rc::new(RefCell::new(NumberType::Pair(
                Rc::new(RefCell::new("5".parse::<NumberType>().unwrap())),
                Rc::clone(&needle),
            ))),
        )));
        assert!(contains_number_type(&haystack, &needle));
    }

    #[test]
    fn test_lowest_literal_down_left_branch() {
        let expected = Rc::new(RefCell::new("7".parse::<NumberType>().unwrap()));
        let literal_haystack = Rc::new(RefCell::new("7".parse::<NumberType>().unwrap()));
        let pair_haystack = Rc::new(RefCell::new("[7, 3]".parse::<NumberType>().unwrap()));
        let deeper_haystack = Rc::new(RefCell::new("[[7, 3], 3]".parse::<NumberType>().unwrap()));
        assert_eq!(lowest_literal_down_left_branch(&literal_haystack), expected);
        assert_eq!(lowest_literal_down_left_branch(&pair_haystack), expected);
        assert_eq!(lowest_literal_down_left_branch(&deeper_haystack), expected);
    }

    #[test]
    fn test_lowest_literal_down_right_branch() {
        let expected = Rc::new(RefCell::new("7".parse::<NumberType>().unwrap()));
        let literal_haystack = Rc::new(RefCell::new("7".parse::<NumberType>().unwrap()));
        let pair_haystack = Rc::new(RefCell::new("[3, 7]".parse::<NumberType>().unwrap()));
        let deeper_haystack = Rc::new(RefCell::new(
            "[[7, 3], [3, 7]]".parse::<NumberType>().unwrap(),
        ));
        assert_eq!(
            lowest_literal_down_right_branch(&literal_haystack),
            expected
        );
        assert_eq!(lowest_literal_down_right_branch(&pair_haystack), expected);
        assert_eq!(lowest_literal_down_right_branch(&deeper_haystack), expected);
    }

    #[test]
    fn test_from_str_literal() {
        assert_eq!("4".parse::<NumberType>().unwrap(), NumberType::Regular(4));
    }

    #[test]
    fn test_from_str_pair() {
        assert_eq!(
            "[3, 7]".parse::<NumberType>().unwrap(),
            NumberType::Pair(
                Rc::new(RefCell::new(NumberType::Regular(3))),
                Rc::new(RefCell::new(NumberType::Regular(7)))
            )
        );
    }

    #[test]
    fn test_from_str_nested_pairs() {
        assert_eq!(
            "[[1, 2], [3, [4, 5]]]".parse::<NumberType>().unwrap(),
            NumberType::Pair(
                Rc::new(RefCell::new(NumberType::Pair(
                    Rc::new(RefCell::new(NumberType::Regular(1))),
                    Rc::new(RefCell::new(NumberType::Regular(2)))
                ))),
                Rc::new(RefCell::new(NumberType::Pair(
                    Rc::new(RefCell::new(NumberType::Regular(3))),
                    Rc::new(RefCell::new(NumberType::Pair(
                        Rc::new(RefCell::new(NumberType::Regular(4))),
                        Rc::new(RefCell::new(NumberType::Regular(5)))
                    )))
                )))
            )
        );
    }

    #[test]
    #[should_panic]
    fn test_from_str_fail_multi_char_digits() {
        let _ = "10".parse::<NumberType>().unwrap();
    }

    #[test]
    fn test_magnitude_calculation() {
        assert_eq!(
            "[[1,2],[[3,4],5]]"
                .parse::<NumberType>()
                .unwrap()
                .magnitude(),
            143
        );
        assert_eq!(
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
                .parse::<NumberType>()
                .unwrap()
                .magnitude(),
            1384
        );
        assert_eq!(
            "[[[[1,1],[2,2]],[3,3]],[4,4]]"
                .parse::<NumberType>()
                .unwrap()
                .magnitude(),
            445
        );
        assert_eq!(
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
                .parse::<NumberType>()
                .unwrap()
                .magnitude(),
            3488
        );
    }
}
