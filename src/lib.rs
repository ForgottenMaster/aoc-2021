pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;

use std::{
    ops::{Add, Shl},
    str::FromStr,
};

/// Struct produced by the "map_windows" function of an Iterator
pub struct MapWindows<T, I, F> {
    wrapped: I,
    func: F,
    window_size: usize,
    window: Vec<T>,
}

impl<T, U, I, F> Iterator for MapWindows<T, I, F>
where
    I: Iterator<Item = T>,
    F: Fn(&[T]) -> U,
{
    type Item = U;

    fn next(&mut self) -> Option<Self::Item> {
        if self.window.len() == 0 {
            // window isn't populated yet, add elements from the underlying
            // iterator until we have self.window_size in the queue (or run out).
            loop {
                if self.window.len() == self.window_size {
                    break Some((self.func)(&self.window));
                } else if let Some(next) = self.wrapped.next() {
                    self.window.push(next);
                } else {
                    break None;
                }
            }
        } else {
            // window was initially populated, simply need to remove the first element
            // and push the new element in.
            if let Some(next) = self.wrapped.next() {
                self.window.remove(0);
                self.window.push(next);
                Some((self.func)(&self.window))
            } else {
                None
            }
        }
    }
}

/// Extension trait to add the "map_windows" adapter to any Iterator
/// that supports it.
pub trait MapWindowsExt<T>: Sized {
    fn map_windows<F>(self, window_size: usize, func: F) -> MapWindows<T, Self, F>;
}

impl<T, S> MapWindowsExt<T> for S
where
    S: Sized,
{
    fn map_windows<F>(self, window_size: usize, func: F) -> MapWindows<T, Self, F> {
        MapWindows {
            wrapped: self,
            func,
            window_size,
            window: Vec::with_capacity(window_size),
        }
    }
}

/// Iterator adapter which takes a predicate and filters out those elements that fail
/// but also uses these as a separator for groups. pushes each element that is part of a group
/// into an internal buffer and whenever a separator (element failing the grouping predicate) is encountered
/// will invoke the mapping function with the slice that has accumulated. The mapping function can then do whatever
/// it needs to with the group. We use a mapping function that takes the slice rather than yielding owned vectors as
/// there may be no need to persist the group. If there is then the mapping function can go ahead and do so.
pub struct FilterGroupMap<T, I, F1, F2> {
    group: Vec<T>,
    iterator: I,
    filter_function: F1,
    map_function: F2,
}

/// Iterator implementation for FilterGroupMap. Will yield the result of applying the map function to the groups
/// determined by the filter_function.
impl<I, F1, F2, U> Iterator for FilterGroupMap<I::Item, I, F1, F2>
where
    I: Iterator,
    F1: Fn(&I::Item) -> bool,
    F2: Fn(&[I::Item]) -> U,
{
    type Item = U;

    fn next(&mut self) -> Option<Self::Item> {
        let mut emit_group = false;
        loop {
            if emit_group {
                let result = (self.map_function)(&self.group);
                self.group.clear();
                break Some(result);
            }

            if let Some(elem) = self.iterator.next() {
                if (self.filter_function)(&elem) {
                    self.group.push(elem);
                } else if !self.group.is_empty() {
                    emit_group = true;
                }
            } else if !self.group.is_empty() {
                emit_group = true;
            } else {
                break None;
            }
        }
    }
}

/// Extension trait which allows us to call filter_map_group on any iterator and produce a decorated iterator that
/// performs the filter, group, and map functionality.
pub trait FilterGroupMapExt<T, I, F1, F2> {
    fn filter_group_map(
        self,
        filter_function: F1,
        map_function: F2,
    ) -> FilterGroupMap<T, I, F1, F2>;
}

/// Blanket implementation of FilterGroupMapExt for all compatible iterators.
impl<I, F1, F2, U> FilterGroupMapExt<I::Item, I, F1, F2> for I
where
    I: Iterator,
    F1: Fn(&I::Item) -> bool,
    F2: Fn(&[I::Item]) -> U,
{
    fn filter_group_map(
        self,
        filter_function: F1,
        map_function: F2,
    ) -> FilterGroupMap<<Self as Iterator>::Item, Self, F1, F2> {
        FilterGroupMap {
            filter_function,
            map_function,
            group: vec![],
            iterator: self,
        }
    }
}

/// Struct which wraps some type which has been parsed from a binary string and allows unwrapping of
/// the value.
#[derive(Debug, PartialEq)]
pub struct ParsedBinaryString<T> {
    parsed: T,
    input_string_bit_count: usize,
}

impl<T> ParsedBinaryString<T> {
    pub fn parsed(&self) -> &T {
        &self.parsed
    }

    pub fn input_string_bit_count(&self) -> usize {
        self.input_string_bit_count
    }
}

impl<T: From<bool> + Shl<u8, Output = T> + Add<T, Output = T>> FromStr for ParsedBinaryString<T> {
    type Err = ParseBinaryStringError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut result: Option<T> = None; // will be None if nothing but whitespace
        let mut input_string_bit_count = 0;

        for (idx, c) in string.chars().enumerate() {
            if c.is_whitespace() {
                continue;
            }

            input_string_bit_count += 1;
            result = match (c, result) {
                ('0', None) => Some(false.into()),
                ('1', None) => Some(true.into()),
                ('0', Some(result)) => Some((result << 1) + false.into()),
                ('1', Some(result)) => Some((result << 1) + true.into()),
                _ => {
                    return Err(ParseBinaryStringError::InvalidChar {
                        string: string.to_string(),
                        index: idx,
                        character: c,
                    })
                }
            };
        }

        if let Some(result) = result {
            Ok(Self {
                parsed: result,
                input_string_bit_count,
            })
        } else {
            Err(ParseBinaryStringError::EmptyString)
        }
    }
}

/// Error cases for the binary string parsing.
#[derive(Debug)]
pub enum ParseBinaryStringError {
    EmptyString,
    InvalidChar {
        string: String,
        index: usize,
        character: char,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsed_binary_string_parsed() {
        let input = ParsedBinaryString {
            parsed: 42,
            input_string_bit_count: 0,
        };
        let expected = &42;
        let calculated = input.parsed();
        assert_eq!(calculated, expected);
    }

    #[test]
    fn test_parsed_binary_string_input_string_bit_count() {
        let input = ParsedBinaryString {
            parsed: 0,
            input_string_bit_count: 42,
        };
        let expected = 42;
        let calculated = input.input_string_bit_count();
        assert_eq!(calculated, expected);
    }

    #[test]
    fn test_parsed_binary_string_empty() {
        assert!(match "     ".parse::<ParsedBinaryString<u32>>() {
            Err(ParseBinaryStringError::EmptyString) => true,
            _ => false,
        });
    }

    #[test]
    fn test_parsed_binary_string_invalid_char() {
        assert!(match "   001 i010   ".parse::<ParsedBinaryString<u32>>() {
            Err(ParseBinaryStringError::InvalidChar {
                string,
                index: 7,
                character: 'i',
            }) if string == "   001 i010   ".to_string() => true,
            _ => false,
        });
    }

    #[test]
    fn test_parsed_binary_string_successful() {
        const INPUT: &str = "      00 101 00   ";
        let expected = ParsedBinaryString {
            parsed: 20,
            input_string_bit_count: 7,
        };
        let calculated = INPUT.parse::<ParsedBinaryString<u32>>().unwrap();
        assert_eq!(calculated, expected);
    }

    #[test]
    fn test_filter_group_empty() {
        const INPUT: &str = r#"
        


        "#;
        let mut iter = INPUT
            .lines()
            .filter_group_map(|line| !line.trim().is_empty(), |group| group.to_owned());
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_filter_group_single_group_multi_line() {
        const INPUT: &str = r#"
        line 1
        line 2
        "#;
        let mut iter = INPUT
            .lines()
            .map(|line| line.trim())
            .filter_group_map(|line| !line.trim().is_empty(), |group| group.to_owned());
        assert_eq!(iter.next().unwrap(), vec!["line 1", "line 2"]);
    }

    #[test]
    fn test_filter_group_multi_group() {
        const INPUT: &str = r#"
        line 1

        line 1
        line 2


        line 1

        line 1
        line 2
        line 3
        "#;
        let mut iter = INPUT
            .lines()
            .map(|line| line.trim())
            .filter_group_map(|line| !line.trim().is_empty(), |group| group.to_owned());
        assert_eq!(iter.next().unwrap(), vec!["line 1"]);
        assert_eq!(iter.next().unwrap(), vec!["line 1", "line 2"]);
        assert_eq!(iter.next().unwrap(), vec!["line 1"]);
        assert_eq!(iter.next().unwrap(), vec!["line 1", "line 2", "line 3"]);
    }
}
