use super::token_type::TokenType;

/// Represents the type of line after parsing. A line can either be:
/// Complete => The line is formatted correctly
/// Incomplete => The line is missing closing characters
/// Corrupted => The line has an incorrect closing character for an opening character
///
/// If the line is incomplete then it will contain the next expected closing TokenType
/// And if it's corrupted will contain both the expected closing TokenType, and the invalid TokenType
#[derive(Debug, PartialEq)]
pub enum LineType {
    Complete,
    Incomplete(Vec<TokenType>),
    Corrupted {
        expected: TokenType,
        found: TokenType,
    },
}
