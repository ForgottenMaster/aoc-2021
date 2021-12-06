use {
    super::{
        bingo_board::BingoBoard, call_sequence::CallSequence,
        read_bingo_game_error::ReadBingoGameError,
    },
    crate::common::iter::FilterGroupMapExt,
    std::io::BufRead,
};

/// Struct representing the entire game of bingo.
#[derive(Clone, Debug, PartialEq)]
pub struct BingoGame(CallSequence, Vec<BingoBoard>);

impl BingoGame {
    pub fn new_from_bufread(reader: impl BufRead) -> Result<Self, ReadBingoGameError> {
        let mut grouped_strings_iter = reader
            .lines()
            .filter_map(|line| line.ok())
            .filter_group_map(
                |line| !line.trim().is_empty(),
                |group| {
                    group
                        .into_iter()
                        .cloned()
                        .collect::<Vec<String>>()
                        .join("\n")
                },
            );
        if let Some(call_sequence) = grouped_strings_iter.next() {
            let call_sequence = call_sequence
                .parse::<CallSequence>()
                .map_err(|err| ReadBingoGameError::ParseCallSequenceError(err))?;
            let mut boards = Vec::new();
            for board in grouped_strings_iter.map(|board| board.parse::<BingoBoard>()) {
                boards.push(board.map_err(|err| ReadBingoGameError::ParseBingoBoardError(err))?);
            }
            Ok(Self(call_sequence, boards))
        } else {
            Err(ReadBingoGameError::InsufficientGroups)
        }
    }

    fn find_winning_board(
        &mut self,
        modify_key_func: impl Fn(u32) -> u32,
    ) -> Option<(&BingoBoard, u32)> {
        (&mut self.1)
            .into_iter()
            .map(|board| (board.run_call_sequence(&self.0), board))
            .filter_map(|(number, board)| {
                if number.is_some() {
                    Some((number.unwrap(), board))
                } else {
                    None
                }
            })
            .min_by_key(|(number, _)| modify_key_func(*number))
            .map(|(number, board)| {
                (
                    &*board,
                    *self.0.into_iter().skip(number as usize - 1).next().unwrap(),
                )
            })
    }

    fn calculate_answer(&mut self, modify_key_func: impl Fn(u32) -> u32) -> u32 {
        let (board, called_number) = self.find_winning_board(modify_key_func).unwrap();
        let remaining_sum = board.sum_unmarked();
        remaining_sum * called_number
    }

    pub fn calculate_part_1_answer(&mut self) -> u32 {
        self.calculate_answer(|num| num)
    }

    pub fn calculate_part_2_answer(&mut self) -> u32 {
        let number_count = self.0.into_iter().count() as u32;
        self.calculate_answer(|num| number_count - num)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bingo_game_call_sequence_error() {
        const INPUT: &[u8] = r#"
        1, 42, i3
        "#
        .as_bytes();
        assert!(match BingoGame::new_from_bufread(INPUT) {
            Err(ReadBingoGameError::ParseCallSequenceError(_)) => true,
            _ => false,
        });
    }

    #[test]
    fn test_bingo_game_bingo_board_error() {
        const INPUT: &[u8] = r#"
        1, 42, 34

        7 8 9
        6 4
        "#
        .as_bytes();
        assert!(match BingoGame::new_from_bufread(INPUT) {
            Err(ReadBingoGameError::ParseBingoBoardError(_)) => true,
            _ => false,
        });
    }

    #[test]
    fn test_bingo_game_insufficient_groups() {
        const INPUT: &[u8] = r#"
        "#
        .as_bytes();
        assert!(match BingoGame::new_from_bufread(INPUT) {
            Err(ReadBingoGameError::InsufficientGroups) => true,
            _ => false,
        });
    }

    #[test]
    fn test_bingo_game_successful() {
        const INPUT: &[u8] = r#"
        1, 42, 34

        7 8 9
        1 2 3



        11 19 14
        22 24 65
        "#
        .as_bytes();
        BingoGame::new_from_bufread(INPUT).unwrap();
    }

    #[test]
    fn test_bingo_game_find_first_winning_board_unsuccessful() {
        const INPUT: &[u8] = r#"
        1, 5, 8, 12, 3
        
        1 2 3
        4 5 6
        
        7 8 9
        10 11 12
        "#
        .as_bytes();
        let mut game = BingoGame::new_from_bufread(INPUT).unwrap();
        assert!(game.find_winning_board(|num| num).is_none());
    }

    #[test]
    fn test_bingo_game_find_first_winning_board_successful() {
        const INPUT: &[u8] = r#"
        1, 5, 8, 12, 3, 11
        
        1 2 3
        4 5 6
        
        7 8 9
        10 11 12
        "#
        .as_bytes();

        let mut game = BingoGame::new_from_bufread(INPUT).unwrap();

        let mut board = r#"
        7 8 9
        10 11 12
        "#
        .parse::<BingoBoard>()
        .unwrap();

        let sequence = "1, 5, 8, 12, 3, 11".parse::<CallSequence>().unwrap();

        assert!(match board.run_call_sequence(&sequence) {
            Some(6) => true,
            _ => false,
        });

        assert_eq!(game.find_winning_board(|num| num).unwrap(), (&board, 11));
    }
}
