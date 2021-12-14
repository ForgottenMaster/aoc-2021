mod bingo_board;
mod bingo_board_entry;
mod bingo_board_row;
mod bingo_game;
mod call_sequence;
mod parse_bingo_board_error;
mod parse_bingo_board_row_error;
mod parse_call_sequence_error;
mod read_bingo_game_error;

use {
    bingo_game::BingoGame,
    std::{fmt::Display, io::BufReader},
};

pub fn run(input: &str) -> (impl Display, impl Display) {
    let input = input.as_bytes();
    let reader = BufReader::new(input);
    let mut bingo_game =
        BingoGame::new_from_bufread(reader).expect("Bingo game could not be parsed from input.");
    let part_1 = bingo_game.clone().calculate_part_1_answer();
    let part_2 = bingo_game.calculate_part_2_answer();
    (part_1, part_2)
}

#[cfg(test)]
mod tests {
    use {super::super::example_input::EXAMPLE_INPUT, super::bingo_game::BingoGame};

    #[test]
    fn test_part_1_example() {
        const INPUT: &[u8] = EXAMPLE_INPUT[3].as_bytes();
        let mut game = BingoGame::new_from_bufread(INPUT).unwrap();
        assert_eq!(game.calculate_part_1_answer(), 4512);
    }
}
