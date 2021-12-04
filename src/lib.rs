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

impl<T: Default + From<bool> + Shl<u8, Output = T> + Add<T, Output = T>> FromStr
    for ParsedBinaryString<T>
{
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
}
