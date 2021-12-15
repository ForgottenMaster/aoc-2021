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
    use super::bingo_game::BingoGame;

    #[test]
    fn test_part_1_example() {
        const INPUT: &[u8] = r#"
        7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

        22 13 17 11  0
        8  2 23  4 24
        21  9 14 16  7
        6 10  3 18  5
        1 12 20 15 19

        3 15  0  2 22
        9 18 13 17  5
        19  8  7 25 23
        20 11 10 24  4
        14 21 16 12  6

        14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
        2  0 12  3  7
        "#
        .as_bytes();
        let mut game = BingoGame::new_from_bufread(INPUT).unwrap();
        assert_eq!(game.calculate_part_1_answer(), 4512);
    }
}
