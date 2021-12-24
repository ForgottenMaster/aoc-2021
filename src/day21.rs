use std::{cmp::min, collections::HashMap, iter::once};

pub fn run(input: &str) -> (u64, u32) {
    let mut iter = input.trim().lines().map(|line| {
        let position = line
            .trim()
            .split(":")
            .skip(1)
            .next()
            .unwrap()
            .trim()
            .parse::<u8>()
            .unwrap();
        Player { position, score: 0 }
    });
    let (player_1, player_2) = (iter.next().unwrap(), iter.next().unwrap());
    let mut universe_hashmap = HashMap::new();
    universe_hashmap.insert((player_1, player_2, NextPlayer::Player1), 1);
    let part_1 = calculate_part_1(universe_hashmap.clone());
    (part_1, 0)
}

/// Identifies the next player's turn to roll.
#[derive(Clone, Eq, Hash, PartialEq)]
enum NextPlayer {
    Player1,
    Player2,
}

/// Allows us to track the current position and score of an individual player.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Player {
    position: u8,
    score: u16,
}

/// Represents a total value of steps for a turn along with the number of universes that
/// will produce those steps.
#[derive(Debug, PartialEq)]
struct DieResultUniverseCount {
    result: u16,
    universes: u8,
}

/// An iterator that can produce the series of 3-roll totals for the deterministic die.
struct DeterministicDieStream {
    next: u8,
}

impl DeterministicDieStream {
    fn roll(&mut self) -> u8 {
        let next = self.next;
        self.next = if self.next == 100 { 1 } else { self.next + 1 };
        next
    }
}

impl Iterator for DeterministicDieStream {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.roll() as u16 + self.roll() as u16 + self.roll() as u16)
    }
}

/// Calculates the result for part 1 of the puzzle. This uses deterministic die stream
/// and runs the game up until a player hits 1000 points minimum. Once that's happened
/// we return the losing players score multiplied by the number of die rolls.
fn calculate_part_1(universe_hashmap: HashMap<(Player, Player, NextPlayer), u64>) -> u64 {
    let mut loser_score = 0;
    let die_rolls = run_game(
        universe_hashmap,
        deterministic_die_stream(),
        1000,
        |(player_1, player_2), _| {
            loser_score = min(player_1.score, player_2.score);
        },
    );
    loser_score as u64 * die_rolls
}

/// Runs the game with the given universes HashMap and roll stream. Will continually process
/// the universe mapping until it's empty. it'll take an entry from the mapping, step the
/// iterator and use the resulting iterator to make new universes, or invoke the callback function.
/// If a game state is a winner then the callback is invoked and it's not put into the hashmap again.
/// If a game state isn't a winner then it's put into the hashmap. Returns the number of times the die was
/// actually rolled.
fn run_game(
    mut universe_hashmap: HashMap<(Player, Player, NextPlayer), u64>,
    mut die_stream: impl Iterator<Item = impl Iterator<Item = DieResultUniverseCount>>,
    winning_score: u64,
    mut win_func: impl FnMut((Player, Player), u64),
) -> u64 {
    let mut die_count = 0;
    while !universe_hashmap.is_empty() {
        let player_state = (*universe_hashmap.keys().next().unwrap()).clone();
        let universes = universe_hashmap.remove(&player_state).unwrap();
        let (player_1_state, player_2_state, next_player) = player_state;
        die_count += 3;

        die_stream.next().unwrap().for_each(|result| {
            let (player_1_state, player_2_state) =
                apply_result(&player_1_state, &player_2_state, &next_player, &result);

            let total_universes = universes * result.universes as u64;

            if player_1_state.score as u64 >= winning_score
                || player_2_state.score as u64 >= winning_score
            {
                win_func((player_1_state, player_2_state), total_universes);
            } else {
                let next_player = match next_player {
                    NextPlayer::Player1 => NextPlayer::Player2,
                    NextPlayer::Player2 => NextPlayer::Player1,
                };
                let entry = universe_hashmap
                    .entry((player_1_state, player_2_state, next_player))
                    .or_insert(0);
                *entry += total_universes;
            }
        });
    }
    die_count
}

/// Takes two players, a next player enum, and the die roll result and will produce two
/// new player states with the position updated/points increased.
fn apply_result(
    player_1_state: &Player,
    player_2_state: &Player,
    next_player: &NextPlayer,
    die_result: &DieResultUniverseCount,
) -> (Player, Player) {
    let (mut player_1_state, mut player_2_state) = (player_1_state.clone(), player_2_state.clone());
    let next_player = match next_player {
        NextPlayer::Player1 => &mut player_1_state,
        NextPlayer::Player2 => &mut player_2_state,
    };
    next_player.position = (((next_player.position as u16 - 1 + die_result.result) % 10) + 1) as u8; // board is from range 1-10 but we want to use mod so need to get into range 0-9
    next_player.score += next_player.position as u16;
    (player_1_state, player_2_state)
}

/// Produces the sequence of steps moved for the deterministic die used in part 1.
/// This only ever produces a single universe per turn which represents the
/// single value that's guaranteed by the 3 die rolls.
fn deterministic_die_stream() -> impl Iterator<Item = impl Iterator<Item = DieResultUniverseCount>>
{
    DeterministicDieStream { next: 1 }.map(|result| {
        once(DieResultUniverseCount {
            result,
            universes: 1,
        })
    })
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

    #[test]
    fn test_deterministic_die_stream_roll() {
        let mut stream = DeterministicDieStream { next: 12 };
        assert_eq!(stream.roll(), 12);
        assert_eq!(stream.roll(), 13);
        assert_eq!(stream.roll(), 14);
        stream.next = 99;
        assert_eq!(stream.roll(), 99);
        assert_eq!(stream.roll(), 100);
        assert_eq!(stream.roll(), 1);
    }

    #[test]
    fn test_deterministic_die_stream_next() {
        let mut stream = DeterministicDieStream { next: 1 };
        assert_eq!(stream.next().unwrap(), 6);
        assert_eq!(stream.next().unwrap(), 15);
        assert_eq!(stream.next().unwrap(), 24);
        stream.next = 99;
        assert_eq!(stream.next().unwrap(), 200);
    }

    #[test]
    fn test_deterministic_die_stream() {
        let mut stream = deterministic_die_stream();
        assert_eq!(
            stream.next().unwrap().next().unwrap(),
            DieResultUniverseCount {
                result: 6,
                universes: 1
            }
        );
        assert_eq!(
            stream.next().unwrap().next().unwrap(),
            DieResultUniverseCount {
                result: 15,
                universes: 1
            }
        );
    }
}
