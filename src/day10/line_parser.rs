use {
    super::{line_type::LineType, token::Token, token_type::TokenType},
    crate::common::collections::Stack,
};

/// Allows parsing lines of text into a final LineType. The reason
/// we use this rather than using the FromStr trait and parse function is so
/// we can reuse the allocation to track the stack.
pub struct LineParser<T> {
    stack: T,
}

impl<T> LineParser<T> {
    pub fn new(stack: T) -> Self {
        Self { stack }
    }
}

/// Placing a Stack bound on T allows us to use a LineParser with any Stack implementation
/// and we only have access to the methods of Stack and that's it. It must be a Stack of Tokens
/// to be usable for parsing.
impl<S: Stack<TokenType>> LineParser<S> {
    pub fn parse(&mut self, chars: impl Iterator<Item = char>) -> LineType {
        self.stack.clear();
        for c in chars {
            let token = Token::from(c); // will panic if invalid character
            let is_opening = token.is_opening_token();
            let token = token.into_token_type();

            if is_opening {
                self.stack.push(token);
            } else {
                let expected = self.stack.pop().unwrap(); // may panic if a closing token appears before an opening one
                if expected != token {
                    return LineType::Corrupted {
                        expected,
                        found: token,
                    };
                }
            }
        }

        if let Some(expected) = self.stack.pop() {
            LineType::Incomplete(expected)
        } else {
            LineType::Complete
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_parser_construction() {
        let parser = LineParser::new(vec![1, 2, 3]);
        assert_eq!(parser.stack.len(), 3);
        assert_eq!(parser.stack[0], 1);
        assert_eq!(parser.stack[1], 2);
        assert_eq!(parser.stack[2], 3);
    }

    #[test]
    fn test_line_parser_parse_complete() {
        let mut parser = LineParser::new(Vec::new());
        assert!(matches!(
            parser.parse("([({})]){}<>[{()}]".chars()),
            LineType::Complete
        ));
    }

    #[test]
    fn test_line_parser_parse_incomplete() {
        let mut parser = LineParser::new(Vec::new());
        assert!(matches!(
            parser.parse("({})[({})][".chars()),
            LineType::Incomplete(TokenType::SquareBracket)
        ));
        assert!(matches!(
            parser.parse("({})[({})](".chars()),
            LineType::Incomplete(TokenType::Parenthesis)
        ));
        assert!(matches!(
            parser.parse("({})[({})]<".chars()),
            LineType::Incomplete(TokenType::AngularBracket)
        ));
        assert!(matches!(
            parser.parse("({})[({})]{".chars()),
            LineType::Incomplete(TokenType::Brace)
        ));
    }

    #[test]
    fn test_line_parser_parse_corrupted() {
        let mut parser = LineParser::new(Vec::new());
        assert!(matches!(
            parser.parse("[({})]<{[)}>".chars()),
            LineType::Corrupted {
                expected: TokenType::SquareBracket,
                found: TokenType::Parenthesis
            }
        ));
        assert!(matches!(
            parser.parse("[({})]{{>}<{[]}>".chars()),
            LineType::Corrupted {
                expected: TokenType::Brace,
                found: TokenType::AngularBracket
            }
        ));
        assert!(matches!(
            parser.parse("[({}}]<{[]}>".chars()),
            LineType::Corrupted {
                expected: TokenType::Parenthesis,
                found: TokenType::Brace
            }
        ));
    }
}
