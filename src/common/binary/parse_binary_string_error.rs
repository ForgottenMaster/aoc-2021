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
