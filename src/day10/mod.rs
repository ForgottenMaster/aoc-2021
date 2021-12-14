mod line_parser;
mod line_type;
mod opening_type;
mod token;
mod token_type;

use {
    line_parser::LineParser, line_type::LineType, std::fs::read_to_string, token_type::TokenType,
};

#[derive(Debug)]
pub enum ExecutionError {}

pub fn run() -> (u32, u64) {
    let input = read_to_string("input/day10.txt").expect("Could not read input file.");
    let (part_1, part_2) = calculate_scores(&input);
    (part_1, part_2)
}

fn calculate_scores(input: &str) -> (u32, u64) {
    let (part_1, mut autocomplete_scores) = parse_lines(input.trim().lines()).filter(|elem| !matches!(elem, LineType::Complete)).fold((0, vec![]), |(mut corrupted_sum, mut autocomplete_scores), elem| {
        match elem {
            LineType::Corrupted {
                found,
                ..
            } => {
                corrupted_sum += score_token_type_corrupted(found);
            },
            LineType::Incomplete(vec) => {
                autocomplete_scores.push(calculate_total_autocomplete_score(vec.into_iter()));
            },
            _ => panic!("This should have been filtered out. We're only interested in corrupted or incomplete lines.")
        }
        (corrupted_sum, autocomplete_scores)
    });
    autocomplete_scores.sort();
    let part_2 = autocomplete_scores[autocomplete_scores.len() / 2];
    (part_1, part_2)
}

fn parse_lines<'a>(
    lines: impl Iterator<Item = &'a str> + 'a,
) -> impl Iterator<Item = LineType> + 'a {
    let mut parser = LineParser::new(Vec::new());
    lines.filter_map(move |line| {
        let line = line.trim();
        if line.is_empty() {
            None
        } else {
            Some(parser.parse(line.chars()))
        }
    })
}

fn score_token_type_corrupted(token_type: TokenType) -> u32 {
    match token_type {
        TokenType::Parenthesis => 3,
        TokenType::SquareBracket => 57,
        TokenType::Brace => 1197,
        TokenType::AngularBracket => 25137,
    }
}

fn score_token_type_incomplete(token_type: TokenType) -> u64 {
    match token_type {
        TokenType::Parenthesis => 1,
        TokenType::SquareBracket => 2,
        TokenType::Brace => 3,
        TokenType::AngularBracket => 4,
    }
}

fn calculate_total_autocomplete_score(input: impl Iterator<Item = TokenType>) -> u64 {
    input.fold(0, |state, elem| {
        state * 5 + score_token_type_incomplete(elem)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_lines() {
        const INPUT: &str = r#"
        
        (({[]}))<{}>
        {[}<[]>[()]
        (({})){[
        
        "#;
        let expected = vec![
            LineType::Complete,
            LineType::Corrupted {
                expected: TokenType::SquareBracket,
                found: TokenType::Brace,
            },
            LineType::Incomplete(vec![TokenType::SquareBracket, TokenType::Brace]),
        ];
        assert_eq!(parse_lines(INPUT.lines()).collect::<Vec<_>>(), expected);
    }

    #[test]
    fn test_part_1_example_parsing() {
        const INPUT: &str = r#"
        [({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]
        "#;
        let expected = vec![
            LineType::Corrupted {
                expected: TokenType::SquareBracket,
                found: TokenType::Brace,
            },
            LineType::Corrupted {
                expected: TokenType::SquareBracket,
                found: TokenType::Parenthesis,
            },
            LineType::Corrupted {
                expected: TokenType::Parenthesis,
                found: TokenType::SquareBracket,
            },
            LineType::Corrupted {
                expected: TokenType::AngularBracket,
                found: TokenType::Parenthesis,
            },
            LineType::Corrupted {
                expected: TokenType::SquareBracket,
                found: TokenType::AngularBracket,
            },
        ];
        assert_eq!(
            parse_lines(INPUT.lines())
                .filter(|elem| matches!(elem, LineType::Corrupted { .. }))
                .collect::<Vec<_>>(),
            expected
        );
    }

    #[test]
    fn test_score_token_type_corrupted() {
        assert_eq!(score_token_type_corrupted(TokenType::Parenthesis), 3);
        assert_eq!(score_token_type_corrupted(TokenType::SquareBracket), 57);
        assert_eq!(score_token_type_corrupted(TokenType::Brace), 1197);
        assert_eq!(score_token_type_corrupted(TokenType::AngularBracket), 25137);
    }

    #[test]
    fn test_score_token_type_incomplete() {
        assert_eq!(score_token_type_incomplete(TokenType::Parenthesis), 1);
        assert_eq!(score_token_type_incomplete(TokenType::SquareBracket), 2);
        assert_eq!(score_token_type_incomplete(TokenType::Brace), 3);
        assert_eq!(score_token_type_incomplete(TokenType::AngularBracket), 4);
    }

    #[test]
    fn test_example_score_calculations() {
        const INPUT: &str = r#"
        [({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]
        "#;
        const EXPECTED: (u32, u64) = (26397, 288957);
        assert_eq!(calculate_scores(INPUT), EXPECTED);
    }

    #[test]
    fn test_calculate_autocomplete_score() {
        const INPUT: &[TokenType] = &[
            TokenType::SquareBracket,
            TokenType::Parenthesis,
            TokenType::Brace,
            TokenType::AngularBracket,
        ];
        const EXPECTED: u64 = 294;
        assert_eq!(
            calculate_total_autocomplete_score(INPUT.into_iter().cloned()),
            EXPECTED
        );
    }
}
