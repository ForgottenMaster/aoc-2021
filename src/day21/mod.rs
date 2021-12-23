mod board;
mod deterministic_roll;
mod die;
mod pawn;
mod roll;

use board::Board;
use deterministic_roll::DeterministicRoll;
use die::Die;
use pawn::Pawn;
use roll::Roll;

pub fn run(input: &str) -> (u64, u32) {
    let mut pawns = input
        .trim()
        .lines()
        .map(|line| {
            Pawn::new(
                line.trim()
                    .split(":")
                    .skip(1)
                    .next()
                    .unwrap()
                    .trim()
                    .parse::<u8>()
                    .unwrap(),
            )
        })
        .collect::<Vec<_>>();
    let board = Board::new(1, 10);
    let mut die = Die::new(DeterministicRoll::new(1, 100));
    while pawns.iter().all(|pawn| pawn.score() < 1000) {
        for pawn in pawns.iter_mut() {
            pawn.apply_steps(&board, (0..3).map(|_| die.roll()).sum());
            if pawn.score() >= 1000 {
                break;
            }
        }
    }
    let part_1 = pawns.iter().map(|pawn| pawn.score()).min().unwrap() * die.count();
    (part_1, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        const INPUT: &str = "
        Player 1 starting position: 4
        Player 2 starting position: 8
        ";
        const EXPECTED: (u64, u32) = (739785, 0);
        assert_eq!(run(INPUT), EXPECTED);
    }
}
