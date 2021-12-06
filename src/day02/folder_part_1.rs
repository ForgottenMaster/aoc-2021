use super::{command::Command, direction::Direction, folder::Folder};

#[derive(Default)]
pub struct FolderPart1 {
    horizontal: u32,
    depth: u32,
}

impl Folder for FolderPart1 {
    type Output = u32;

    fn apply(self, command: Command) -> Self {
        match command.direction() {
            Direction::Forward => Self {
                horizontal: self.horizontal + command.amount(),
                depth: self.depth,
            },
            Direction::Down => Self {
                horizontal: self.horizontal,
                depth: self.depth + command.amount(),
            },
            Direction::Up => Self {
                horizontal: self.horizontal,
                depth: if self.depth >= command.amount() {
                    self.depth - command.amount()
                } else {
                    0
                },
            },
        }
    }

    fn unwrap(self) -> Self::Output {
        self.horizontal * self.depth
    }
}
