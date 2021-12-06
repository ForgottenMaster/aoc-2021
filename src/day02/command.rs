use {super::direction::Direction, std::str::FromStr};

#[derive(Clone, Debug)]
pub struct Command {
    direction: Direction,
    amount: u32,
}

impl Command {
    pub fn direction(&self) -> &Direction {
        &self.direction
    }

    pub fn amount(&self) -> u32 {
        self.amount
    }
}

impl FromStr for Command {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut split = string.trim().split(" ");
        let direction_string = split.next().ok_or(())?;
        let amount_string = split.next().ok_or(())?;
        let direction = direction_string.parse()?;
        let amount = amount_string.parse().map_err(|_| ())?;

        Ok(Command { direction, amount })
    }
}
