use {
    super::{
        bingo_board::BingoBoard, call_sequence::CallSequence,
        read_bingo_game_error::ReadBingoGameError,
    },
    crate::common::iter::FilterGroupMapExt,
    std::io::BufRead,
};

/// Struct representing the entire game of bingo.
#[derive(Debug, PartialEq)]
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
}
