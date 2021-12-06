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
    std::{error::Error, fmt::Display, fs::File, io::BufReader},
};

pub fn run() -> Result<(Box<dyn Display>, Box<dyn Display>), Box<dyn Error>> {
    let file = File::open("input/day04.txt")?;
    let reader = BufReader::new(file);
    let _bingo_game = BingoGame::new_from_bufread(reader)?;
    Ok((Box::new(0), Box::new(0)))
}
