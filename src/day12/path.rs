use {
    super::node::Node,
    std::collections::{HashMap, HashSet},
};

/// Struct encapsulating the concept of a path through the nodes
/// which is just a wrapper around a Vec<Node> sequence. Has a method
/// called Proceed which takes a reference to the HashMap containing the
/// links between nodes in order to produce the replacement paths for self.
/// This allows us to track branching paths as when a single path is proceeded
/// it will return multiple new paths for each neighbour it can visit. A path is always initialized
/// publicly with the only node in the sequence being the start node.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Path {
    sequence: Vec<Node>,
    small_cave_hashes_visited: Vec<u64>,
}

impl Path {
    /// Creates a new path that initially has only "Start" in it.
    pub fn new() -> Self {
        Self {
            sequence: vec![Node::Start],
            small_cave_hashes_visited: vec![],
        }
    }

    /// Determines if the path is complete or not.
    pub fn is_complete(&self) -> bool {
        self.sequence.last() == Some(&Node::End)
    }

    /// Consumes self and returns a vector of replacement paths.
    /// If there are no paths that can be reached from self then None is returned.
    /// Returns a vector so we can support the path branching up into all its neighbours.
    pub fn proceed(self, links: &HashMap<Node, HashSet<Node>>) -> Option<Vec<Self>> {
        // if it's complete already then there's nowhere to go so just return self again.
        if self.is_complete() {
            Some(vec![self])
        } else {
            let paths: Vec<Self> = links[self.sequence.last().unwrap()]
                .iter()
                .filter_map(|node| {
                    if let Node::Start = node {
                        None
                    } else {
                        if let Node::SmallCave(hash) = node {
                            if self.small_cave_hashes_visited.contains(hash) {
                                return None;
                            }
                        }

                        // New path will branch from this one, so the visited caves and sequence will be the same
                        let mut sequence = self.sequence.clone();
                        let mut small_cave_hashes_visited = self.small_cave_hashes_visited.clone();

                        // Except that we push on the new node
                        sequence.push((*node).clone());
                        if let Node::SmallCave(hash) = node {
                            small_cave_hashes_visited.push(*hash);
                        }

                        Some(Self {
                            sequence,
                            small_cave_hashes_visited,
                        })
                    }
                })
                .collect();

            if paths.is_empty() {
                None
            } else {
                Some(paths)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_construction() {
        let path = Path::new();
        assert_eq!(path.sequence, vec![Node::Start]);
        assert_eq!(path.small_cave_hashes_visited, vec![]);
    }

    #[test]
    fn test_is_complete_check() {
        let mut path = Path::new();
        assert!(!path.is_complete());
        path.sequence.push(Node::End);
        assert!(path.is_complete());
    }

    #[test]
    fn test_proceed_calculation_correct() {
        let mut hm: HashMap<Node, HashSet<Node>> = HashMap::new();
        hm.entry(Node::Start)
            .or_default()
            .insert(Node::SmallCave(22));
        hm.entry(Node::SmallCave(22))
            .or_default()
            .insert(Node::Start);

        hm.entry(Node::Start)
            .or_default()
            .insert(Node::SmallCave(25));
        hm.entry(Node::SmallCave(25))
            .or_default()
            .insert(Node::Start);

        hm.entry(Node::SmallCave(22))
            .or_default()
            .insert(Node::LargeCave(28));
        hm.entry(Node::LargeCave(28))
            .or_default()
            .insert(Node::SmallCave(22));

        hm.entry(Node::LargeCave(28)).or_default().insert(Node::End);
        hm.entry(Node::End).or_default().insert(Node::LargeCave(28));

        let paths = vec![Path::new()];
        let paths = paths
            .into_iter()
            .filter_map(|path| path.proceed(&mut hm))
            .flat_map(|paths| paths.into_iter())
            .collect::<Vec<_>>();

        assert_eq!(
            paths.iter().collect::<HashSet<_>>(),
            vec![
                Path {
                    sequence: vec![Node::Start, Node::SmallCave(22)],
                    small_cave_hashes_visited: [22].into_iter().collect()
                },
                Path {
                    sequence: vec![Node::Start, Node::SmallCave(25)],
                    small_cave_hashes_visited: [25].into_iter().collect()
                }
            ]
            .iter()
            .collect::<HashSet<_>>()
        );

        let paths = paths
            .into_iter()
            .filter_map(|path| path.proceed(&mut hm))
            .flat_map(|paths| paths.into_iter())
            .collect::<Vec<_>>();
        assert_eq!(
            paths.iter().collect::<HashSet<_>>(),
            vec![Path {
                sequence: vec![Node::Start, Node::SmallCave(22), Node::LargeCave(28)],
                small_cave_hashes_visited: [22].into_iter().collect()
            }]
            .iter()
            .collect::<HashSet<_>>()
        );

        let paths = paths
            .into_iter()
            .filter_map(|path| path.proceed(&mut hm))
            .flat_map(|paths| paths.into_iter())
            .collect::<Vec<_>>();
        assert_eq!(
            paths.iter().collect::<HashSet<_>>(),
            vec![Path {
                sequence: vec![
                    Node::Start,
                    Node::SmallCave(22),
                    Node::LargeCave(28),
                    Node::End
                ],
                small_cave_hashes_visited: [22].into_iter().collect()
            }]
            .iter()
            .collect::<HashSet<_>>()
        );

        // Check completed paths remain completed.
        let paths = paths
            .into_iter()
            .filter_map(|path| path.proceed(&mut hm))
            .flat_map(|paths| paths.into_iter())
            .collect::<Vec<_>>();
        assert_eq!(
            paths.iter().collect::<HashSet<_>>(),
            vec![Path {
                sequence: vec![
                    Node::Start,
                    Node::SmallCave(22),
                    Node::LargeCave(28),
                    Node::End
                ],
                small_cave_hashes_visited: [22].into_iter().collect()
            }]
            .iter()
            .collect::<HashSet<_>>()
        );
    }
}
