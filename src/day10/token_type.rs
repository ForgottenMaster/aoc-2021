/// The category/type of the token which can be one of 4 types
#[derive(Debug, PartialEq)]
pub enum TokenType {
    Parenthesis,
    SquareBracket,
    Brace,
    AngularBracket,
}
