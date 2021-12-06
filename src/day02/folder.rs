use super::command::Command;

/// This is a trait that allows a command to be applied in a fold for a command sequence
/// and will update its own internal state with the given command in apply. When unwrap is called
/// will produce its output.
pub trait Folder {
    type Output;

    fn apply(self, command: Command) -> Self;
    fn unwrap(self) -> Self::Output;
}
