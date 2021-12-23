use super::Roll;

/// Die is basically just a wrapper around another Roll implementation
/// but which also tracks the number of times it's been rolled.
#[derive(Debug, PartialEq)]
pub struct Die<T> {
    roll: T,
    count: u64,
}

impl<T> Die<T> {
    /// Creates a new instance of Die with the given roll implementation, starting at
    /// 0 roll count.
    pub fn new(roll: T) -> Self {
        Self { roll, count: 0 }
    }

    /// Gets the current roll count from the Die.
    pub fn count(&self) -> u64 {
        self.count
    }
}

/// Implement Roll for Die since it's just a wrapper around Roll itself.
impl<T: Roll> Roll for Die<T> {
    fn roll(&mut self) -> u16 {
        self.count += 1;
        self.roll.roll()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct FixedRoll;

    impl Roll for FixedRoll {
        fn roll(&mut self) -> u16 {
            42
        }
    }

    #[test]
    fn test_die_new() {
        let expected = Die {
            roll: FixedRoll,
            count: 0,
        };
        let calculated = Die::new(FixedRoll);
        assert_eq!(expected, calculated);
    }

    #[test]
    fn test_die_count() {
        let mut die = Die::new(FixedRoll);
        assert_eq!(die.count(), 0);
        die.roll();
        die.roll();
        die.roll();
        die.roll();
        assert_eq!(die.count(), 4);
    }

    #[test]
    fn test_die_roll() {
        let mut die = Die::new(FixedRoll);
        assert_eq!(die.roll(), 42);
        assert_eq!(die.roll(), 42);
        assert_eq!(die.roll(), 42);
        assert_eq!(die.count, 3);
    }
}
