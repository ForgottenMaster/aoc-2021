/// Struct representing a single Lanternfish which has an internal timer
/// that cycles around if it is ticked while at 0 already.
#[derive(Debug, PartialEq)]
pub struct Lanternfish(u8);

impl Lanternfish {
    /// Ticks the lifetime of the Lanternfish by 1 tick, resetting
    /// to the given parameter if it's at 0. The return value is a new fish
    /// if one was spawned, else None.
    pub fn tick(&mut self, reset_to: u8, new_spawn_at: u8) -> Option<Self> {
        if self.0 == 0 {
            self.0 = reset_to;
            Some(Self(new_spawn_at))
        } else {
            self.0 -= 1;
            None
        }
    }
}

impl From<u8> for Lanternfish {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lanterfish_from_integer() {
        let expected = Lanternfish(12);
        let calculated: Lanternfish = 12.into();
        assert_eq!(expected, calculated);
    }

    #[test]
    fn test_lanternfish_tick_without_cycle() {
        let mut fish = Lanternfish(12);
        assert!(fish.tick(12, 14).is_none());
        let expected = Lanternfish(11);
        assert_eq!(fish, expected);
    }

    #[test]
    fn test_lanternfish_tick_to_0() {
        let mut fish = Lanternfish(1);
        assert!(fish.tick(12, 14).is_none());
        let expected = Lanternfish(0);
        assert_eq!(fish, expected);
    }

    #[test]
    fn test_lanternfish_tick_reset() {
        let mut fish = Lanternfish(0);
        assert_eq!(fish.tick(12, 14).unwrap().0, 14);
        let expected = Lanternfish(12);
        assert_eq!(fish, expected);
    }
}
