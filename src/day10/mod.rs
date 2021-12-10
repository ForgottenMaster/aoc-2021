mod line_parser;
mod line_type;
mod opening_type;
mod token;
mod token_type;

use {
    line_parser::LineParser, line_type::LineType, std::fs::read_to_string, token_type::TokenType,
};

pub fn run() -> (u32, u32) {
    let input = read_to_string("input/day10.txt").expect("Could not read input file.");
    let part_1 = calculate_part_1(&input);
    (part_1, 0)
}

fn calculate_part_1(input: &str) -> u32 {
    score_corrupted_lines(parse_lines(input.trim().lines())).sum()
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

fn score_corrupted_lines(iter: impl Iterator<Item = LineType>) -> impl Iterator<Item = u32> {
    iter.filter_map(|elem| match elem {
        LineType::Corrupted { found, .. } => Some(score_token_type(found)),
        _ => None,
    })
}

fn score_token_type(token_type: TokenType) -> u32 {
    match token_type {
        TokenType::Parenthesis => 3,
        TokenType::SquareBracket => 57,
        TokenType::Brace => 1197,
        TokenType::AngularBracket => 25137,
    }
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
            LineType::Incomplete(TokenType::SquareBracket),
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
    fn test_score_token_type() {
        assert_eq!(score_token_type(TokenType::Parenthesis), 3);
        assert_eq!(score_token_type(TokenType::SquareBracket), 57);
        assert_eq!(score_token_type(TokenType::Brace), 1197);
        assert_eq!(score_token_type(TokenType::AngularBracket), 25137);
    }

    #[test]
    fn test_score_corrupted_lines() {
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
        let expected = vec![1197, 3, 57, 3, 25137];
        assert_eq!(
            score_corrupted_lines(parse_lines(INPUT.trim().lines())).collect::<Vec<_>>(),
            expected
        );
    }

    #[test]
    fn test_part_1_example_solution_correct() {
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
        const EXPECTED: u32 = 26397;
        assert_eq!(calculate_part_1(INPUT), EXPECTED);
    }
}
