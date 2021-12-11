use super::{opening_type::OpeningType, token_type::TokenType};

/// Represents a single token in the input stream. A token is a single character which can either be
/// an open token or close token of the set.
#[derive(Debug, PartialEq)]
pub struct Token {
    opening_type: OpeningType,
    token_type: TokenType,
}

impl Token {
    pub fn is_opening_token(&self) -> bool {
        matches!(self.opening_type, OpeningType::Opening)
    }

    pub fn into_token_type(self) -> TokenType {
        self.token_type
    }
}

/// Implementation to convert from a character to a Token. As the previous couple of
/// days, we'll just panic if an invalid character is given. It saves the headache of handling
/// errors "neatly" and the issue will be in the input string.
impl From<char> for Token {
    fn from(value: char) -> Self {
        match value {
            '(' => Self {
                opening_type: OpeningType::Opening,
                token_type: TokenType::Parenthesis,
            },
            ')' => Self {
                opening_type: OpeningType::Closing,
                token_type: TokenType::Parenthesis,
            },
            '[' => Self {
                opening_type: OpeningType::Opening,
                token_type: TokenType::SquareBracket,
            },
            ']' => Self {
                opening_type: OpeningType::Closing,
                token_type: TokenType::SquareBracket,
            },
            '{' => Self {
                opening_type: OpeningType::Opening,
                token_type: TokenType::Brace,
            },
            '}' => Self {
                opening_type: OpeningType::Closing,
                token_type: TokenType::Brace,
            },
            '<' => Self {
                opening_type: OpeningType::Opening,
                token_type: TokenType::AngularBracket,
            },
            '>' => Self {
                opening_type: OpeningType::Closing,
                token_type: TokenType::AngularBracket,
            },
            _ => panic!("Tried to convert an invalid character {} to a Token", value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_char_to_token_valid() {
        const INPUT: &[char] = &['(', ')', '[', ']', '{', '}', '<', '>'];
        let expected = vec![
            Token {
                opening_type: OpeningType::Opening,
                token_type: TokenType::Parenthesis,
            },
            Token {
                opening_type: OpeningType::Closing,
                token_type: TokenType::Parenthesis,
            },
            Token {
                opening_type: OpeningType::Opening,
                token_type: TokenType::SquareBracket,
            },
            Token {
                opening_type: OpeningType::Closing,
                token_type: TokenType::SquareBracket,
            },
            Token {
                opening_type: OpeningType::Opening,
                token_type: TokenType::Brace,
            },
            Token {
                opening_type: OpeningType::Closing,
                token_type: TokenType::Brace,
            },
            Token {
                opening_type: OpeningType::Opening,
                token_type: TokenType::AngularBracket,
            },
            Token {
                opening_type: OpeningType::Closing,
                token_type: TokenType::AngularBracket,
            },
        ];
        assert_eq!(
            INPUT
                .into_iter()
                .map(|c| Token::from(*c))
                .collect::<Vec<_>>(),
            expected
        );
    }

    #[test]
    #[should_panic]
    fn test_convert_char_to_token_invalid() {
        let _ = Token::from('z');
    }

    #[test]
    fn test_is_opening_token() {
        let token = Token {
            opening_type: OpeningType::Opening,
            token_type: TokenType::AngularBracket,
        };
        assert!(token.is_opening_token());
        let token = Token {
            opening_type: OpeningType::Closing,
            token_type: TokenType::AngularBracket,
        };
        assert!(!token.is_opening_token());
    }

    #[test]
    fn test_into_token_type() {
        assert_eq!(
            Token {
                opening_type: OpeningType::Opening,
                token_type: TokenType::Parenthesis
            }
            .into_token_type(),
            TokenType::Parenthesis
        );
        assert_eq!(
            Token {
                opening_type: OpeningType::Opening,
                token_type: TokenType::SquareBracket
            }
            .into_token_type(),
            TokenType::SquareBracket
        );
        assert_eq!(
            Token {
                opening_type: OpeningType::Opening,
                token_type: TokenType::Brace
            }
            .into_token_type(),
            TokenType::Brace
        );
        assert_eq!(
            Token {
                opening_type: OpeningType::Opening,
                token_type: TokenType::AngularBracket
            }
            .into_token_type(),
            TokenType::AngularBracket
        );
    }
}
