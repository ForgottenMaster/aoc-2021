/// Determines what the next randomly rolled number should be.
pub trait Roll {
    fn roll(&mut self) -> u16;
}
