use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum Direction {
    Forward,
    Down,
    Up,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string.trim() {
            "forward" => Ok(Direction::Forward),
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            _ => Err(()),
        }
    }
}
