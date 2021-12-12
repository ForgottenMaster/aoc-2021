use std::{collections::hash_map::DefaultHasher, convert::Infallible, hash::Hasher, str::FromStr};

/// Represents the type of node that we're visiting which will either be
/// the special start or end node, or a large or small cave with a specific
/// identifying string. We will hash the identifying string when we parse/load the
/// mapping since it doesn't need to be human-readable and it's faster to process hashed
/// values than string comparison.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Node {
    Start,
    End,
    SmallCave(u64),
    LargeCave(u64),
}

impl FromStr for Node {
    type Err = Infallible; // Just panic on parse error - it's easier

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let string = string.trim();
        Ok(match string {
            "start" => Self::Start,
            "end" => Self::End,
            string => {
                // Determine if it's a lowercase or upper case string, while detecting
                // a mismatch and panicing.
                let is_lowercase = string.chars().fold(None, |is_lowercase, character| {
                    let char_lowercase = character.is_lowercase();
                    let char_uppercase = character.is_uppercase();
                    match (is_lowercase, char_lowercase, char_uppercase) {
                        (_, false, false) => panic!("Invalid character {} found. Node identifiers must be uppercase or lowercase alphabetical characters only.", character),
                        (None, true, _) => Some(true),
                        (None, _, true) => Some(false),
                        (Some(true), false, _) | (Some(false), true, _) => panic!("Character {} found which doesn't match the established case of the node identifier.", character),
                        _ => is_lowercase
                    }
                }).unwrap();

                // If we get here then the string is either all lowercase or all uppercase
                // and we know which one it is. We will store the string hash though not the string
                // itself.
                let mut hasher = DefaultHasher::new();
                hasher.write(string.as_bytes());
                let hash = hasher.finish();

                // We now can emit the correct cave type for the node.
                if is_lowercase {
                    Self::SmallCave(hash)
                } else {
                    Self::LargeCave(hash)
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_from_str_start() {
        assert_eq!("start".parse::<Node>().unwrap(), Node::Start);
    }

    #[test]
    fn test_node_from_str_end() {
        assert_eq!("end".parse::<Node>().unwrap(), Node::End);
    }

    #[test]
    #[should_panic]
    fn test_node_from_str_panic_invalid_character() {
        let _ = "uvh3m".parse::<Node>().unwrap();
    }

    #[test]
    #[should_panic]
    fn test_node_from_str_panic_invalid_casing() {
        let _ = "uvhMno".parse::<Node>().unwrap();
    }

    #[test]
    fn test_node_from_str_small_cave() {
        let mut hasher = DefaultHasher::new();
        hasher.write(b"uvhmr");
        let hash = hasher.finish();
        assert_eq!("uvhmr".parse::<Node>().unwrap(), Node::SmallCave(hash));
    }

    #[test]
    fn test_node_from_str_large_cave() {
        let mut hasher = DefaultHasher::new();
        hasher.write(b"UVHMR");
        let hash = hasher.finish();
        assert_eq!("UVHMR".parse::<Node>().unwrap(), Node::LargeCave(hash));
    }
}
