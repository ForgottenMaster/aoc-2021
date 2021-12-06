use super::{command::Command, direction::Direction, folder::Folder};

#[derive(Default)]
pub struct FolderPart2 {
    horizontal: u32,
    depth: u32,
    aim: u32,
}

impl Folder for FolderPart2 {
    type Output = u32;

    fn apply(self, command: Command) -> Self {
        match command.direction() {
            Direction::Down => Self {
                horizontal: self.horizontal,
                depth: self.depth,
                aim: self.aim + command.amount(),
            },
            Direction::Up => Self {
                horizontal: self.horizontal,
                depth: self.depth,
                aim: if self.aim >= command.amount() {
                    self.aim - command.amount()
                } else {
                    0
                },
            },
            Direction::Forward => Self {
                horizontal: self.horizontal + command.amount(),
                depth: self.depth + self.aim * command.amount(),
                aim: self.aim,
            },
        }
    }

    fn unwrap(self) -> Self::Output {
        self.horizontal * self.depth
    }
}
