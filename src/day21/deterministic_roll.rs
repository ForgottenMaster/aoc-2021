use super::Roll;

/// An implementation of Roll which cycles through integers, from
/// a minimum to a maximum.
#[derive(Debug, PartialEq)]
pub struct DeterministicRoll {
    min: u16,
    max: u16,
    next: u16,
}

impl DeterministicRoll {
    pub fn new(min: u16, max: u16) -> Self {
        Self {
            min,
            max,
            next: min,
        }
    }
}

impl Roll for DeterministicRoll {
    fn roll(&mut self) -> u16 {
        let returned = self.next;
        self.next = if self.next == self.max {
            self.min
        } else {
            self.next + 1
        };
        returned
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deterministic_roll_new() {
        let calculated = DeterministicRoll::new(10, 43);
        let expected = DeterministicRoll {
            min: 10,
            max: 43,
            next: 10,
        };
        assert_eq!(calculated, expected);
    }

    #[test]
    fn test_deterministic_roll_roll() {
        let mut roll = DeterministicRoll::new(5, 10);
        assert_eq!(roll.roll(), 5);
        assert_eq!(roll.roll(), 6);
        assert_eq!(roll.roll(), 7);
        assert_eq!(roll.roll(), 8);
        assert_eq!(roll.roll(), 9);
        assert_eq!(roll.roll(), 10);
        assert_eq!(roll.roll(), 5);
    }
}
