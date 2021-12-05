use {
    implementation::{BingoGame, ReadBingoGameError},
    std::{
        fmt::Display,
        fs::File,
        io::{BufReader, Error},
    },
};

pub enum ExecutionError {
    IOError(Error),
    ReadBingoGameError(ReadBingoGameError),
}

impl From<Error> for ExecutionError {
    fn from(value: Error) -> Self {
        Self::IOError(value)
    }
}

impl From<ReadBingoGameError> for ExecutionError {
    fn from(value: ReadBingoGameError) -> Self {
        Self::ReadBingoGameError(value)
    }
}

pub fn run() -> Result<(Box<dyn Display>, Box<dyn Display>), ExecutionError> {
    let file = File::open("input/day04.txt")?;
    let reader = BufReader::new(file);
    let _bingo_game = BingoGame::new_from_bufread(reader)?;
    Ok((Box::new(0), Box::new(0)))
}

// Put into a module to respect privacy rules
mod implementation {
    use {
        crate::common::iter::FilterGroupMapExt,
        std::{io::BufRead, num::ParseIntError, str::FromStr},
    };

    /// Struct that represents the call sequence for the bingo game.
    #[derive(Debug, PartialEq)]
    struct CallSequence(Vec<u32>);

    #[derive(Debug)]
    pub enum ParseCallSequenceError {
        ParseIntError(ParseIntError),
        EmptySequence,
    }

    impl FromStr for CallSequence {
        type Err = ParseCallSequenceError;

        fn from_str(string: &str) -> Result<Self, Self::Err> {
            let mut sequence = Vec::new();
            for value in string
                .trim()
                .split(",")
                .map(|elem| elem.trim())
                .filter(|elem| !elem.is_empty())
                .map(|elem| elem.parse())
            {
                sequence.push(value.map_err(|err| ParseCallSequenceError::ParseIntError(err))?);
            }

            if sequence.is_empty() {
                Err(ParseCallSequenceError::EmptySequence)
            } else {
                Ok(Self(sequence))
            }
        }
    }

    /// Struct defining a single entry in the bingo board.
    #[derive(Debug, PartialEq)]
    struct BingoBoardEntry {
        value: u32,
        is_marked: bool,
    }

    /// Struct that represents a single row in the bingo game.
    #[derive(Debug, PartialEq)]
    struct BingoBoardRow(Vec<BingoBoardEntry>);

    #[derive(Debug)]
    pub enum ParseBingoBoardRowError {
        ParseIntError(ParseIntError),
        EmptySequence,
    }

    impl FromStr for BingoBoardRow {
        type Err = ParseBingoBoardRowError;

        fn from_str(string: &str) -> Result<Self, Self::Err> {
            let mut sequence = Vec::new();
            for value in string
                .trim()
                .split(" ")
                .map(|elem| elem.trim())
                .filter(|elem| !elem.is_empty())
                .map(|elem| elem.parse())
            {
                sequence.push(BingoBoardEntry {
                    value: value.map_err(|err| ParseBingoBoardRowError::ParseIntError(err))?,
                    is_marked: false,
                });
            }

            if sequence.is_empty() {
                Err(ParseBingoBoardRowError::EmptySequence)
            } else {
                Ok(Self(sequence))
            }
        }
    }

    /// Struct that represents a bingo board in the bingo game.
    #[derive(Debug, PartialEq)]
    struct BingoBoard(Vec<BingoBoardRow>);

    #[derive(Debug)]
    pub enum ParseBingoBoardError {
        BingoBoardRowError(ParseBingoBoardRowError),
        EmptyBoard,
        MismatchingRowLengths,
    }

    impl FromStr for BingoBoard {
        type Err = ParseBingoBoardError;

        fn from_str(string: &str) -> Result<Self, Self::Err> {
            let mut rows = Vec::new();
            let mut row_length = 0;

            for row in string
                .trim()
                .lines()
                .map(|line| line.trim().parse::<BingoBoardRow>())
            {
                let row = row.map_err(|err| ParseBingoBoardError::BingoBoardRowError(err))?;
                if row_length != 0 && row.0.len() != row_length {
                    return Err(ParseBingoBoardError::MismatchingRowLengths);
                }
                row_length = row.0.len();
                rows.push(row);
            }

            if rows.is_empty() {
                Err(ParseBingoBoardError::EmptyBoard)
            } else {
                Ok(Self(rows))
            }
        }
    }

    /// Struct representing the entire game of bingo.
    #[derive(Debug, PartialEq)]
    pub struct BingoGame(CallSequence, Vec<BingoBoard>);

    #[derive(Debug)]
    pub enum ReadBingoGameError {
        ParseCallSequenceError(ParseCallSequenceError),
        ParseBingoBoardError(ParseBingoBoardError),
        InsufficientGroups,
    }

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
                    boards
                        .push(board.map_err(|err| ReadBingoGameError::ParseBingoBoardError(err))?);
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
        fn test_call_sequence_invalid_char() {
            const INPUT: &str = "21,45,i5";
            assert!(match INPUT.parse::<CallSequence>() {
                Err(ParseCallSequenceError::ParseIntError(_)) => true,
                _ => false,
            });
        }

        #[test]
        fn test_call_sequence_empty() {
            const INPUT: &str = "     ";
            assert!(match INPUT.parse::<CallSequence>() {
                Err(ParseCallSequenceError::EmptySequence) => true,
                _ => false,
            });
        }

        #[test]
        fn test_call_sequence_valid() {
            const INPUT: &str = "   42,  365, 18, 24    ";
            let expected = vec![42, 365, 18, 24];
            let calculated = INPUT.parse::<CallSequence>().unwrap().0;
            assert_eq!(calculated, expected);
        }

        #[test]
        fn test_bingo_board_row_invalid_char() {
            const INPUT: &str = "21 45 i5";
            assert!(match INPUT.parse::<BingoBoardRow>() {
                Err(ParseBingoBoardRowError::ParseIntError(_)) => true,
                _ => false,
            });
        }

        #[test]
        fn test_bingo_board_row_empty() {
            const INPUT: &str = "     ";
            assert!(match INPUT.parse::<BingoBoardRow>() {
                Err(ParseBingoBoardRowError::EmptySequence) => true,
                _ => false,
            });
        }

        #[test]
        fn test_bingo_board_row_valid() {
            const INPUT: &str = "   42  365  18  24    ";
            let expected = vec![
                BingoBoardEntry {
                    value: 42,
                    is_marked: false,
                },
                BingoBoardEntry {
                    value: 365,
                    is_marked: false,
                },
                BingoBoardEntry {
                    value: 18,
                    is_marked: false,
                },
                BingoBoardEntry {
                    value: 24,
                    is_marked: false,
                },
            ];
            let calculated = INPUT.parse::<BingoBoardRow>().unwrap().0;
            assert_eq!(calculated, expected);
        }

        #[test]
        fn test_bingo_board_empty() {
            const INPUT: &str = "        ";
            assert!(match INPUT.parse::<BingoBoard>() {
                Err(ParseBingoBoardError::EmptyBoard) => true,
                _ => false,
            });
        }

        #[test]
        fn test_bingo_board_mismatching_rows() {
            const INPUT: &str = r#"
            1 2 3 4
            1 2 3
            "#;
            assert!(match INPUT.parse::<BingoBoard>() {
                Err(ParseBingoBoardError::MismatchingRowLengths) => true,
                _ => false,
            });
        }

        #[test]
        fn test_bingo_board_single_row() {
            const INPUT: &str = "   1 2  3    4    ";
            let expected = BingoBoard(vec![BingoBoardRow(vec![
                BingoBoardEntry {
                    value: 1,
                    is_marked: false,
                },
                BingoBoardEntry {
                    value: 2,
                    is_marked: false,
                },
                BingoBoardEntry {
                    value: 3,
                    is_marked: false,
                },
                BingoBoardEntry {
                    value: 4,
                    is_marked: false,
                },
            ])]);
            let calculated = INPUT.parse::<BingoBoard>().unwrap();
            assert_eq!(expected, calculated);
        }

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
            assert_eq!(
                BingoGame::new_from_bufread(INPUT).unwrap(),
                BingoGame(
                    CallSequence(vec![1, 42, 34]),
                    vec![
                        BingoBoard(vec![
                            BingoBoardRow(vec![
                                BingoBoardEntry {
                                    value: 7,
                                    is_marked: false
                                },
                                BingoBoardEntry {
                                    value: 8,
                                    is_marked: false
                                },
                                BingoBoardEntry {
                                    value: 9,
                                    is_marked: false
                                }
                            ]),
                            BingoBoardRow(vec![
                                BingoBoardEntry {
                                    value: 1,
                                    is_marked: false
                                },
                                BingoBoardEntry {
                                    value: 2,
                                    is_marked: false
                                },
                                BingoBoardEntry {
                                    value: 3,
                                    is_marked: false
                                }
                            ])
                        ]),
                        BingoBoard(vec![
                            BingoBoardRow(vec![
                                BingoBoardEntry {
                                    value: 11,
                                    is_marked: false
                                },
                                BingoBoardEntry {
                                    value: 19,
                                    is_marked: false
                                },
                                BingoBoardEntry {
                                    value: 14,
                                    is_marked: false
                                }
                            ]),
                            BingoBoardRow(vec![
                                BingoBoardEntry {
                                    value: 22,
                                    is_marked: false
                                },
                                BingoBoardEntry {
                                    value: 24,
                                    is_marked: false
                                },
                                BingoBoardEntry {
                                    value: 65,
                                    is_marked: false
                                }
                            ])
                        ])
                    ]
                )
            );
        }
    }
}
