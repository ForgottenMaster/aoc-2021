mod node;

use {
    super::common::collections::Stack,
    node::Node,
    std::{
        borrow::Borrow,
        collections::{
            hash_map::DefaultHasher,
            {HashMap, HashSet},
        },
        fs::read_to_string,
        hash::{Hash, Hasher},
    },
};

pub fn run() -> (usize, usize) {
    let content = read_to_string("input/day12.txt").expect("Could not open file for input.");
    count_paths_through_cave(content.trim().lines())
}

/// Takes an iterator over lines of text which will be parsed as the links of the graph.
/// Determines results for part 1 (not allowing a double visit) and for part 2. (allowing double visit of ONE small cave).
fn count_paths_through_cave(iter: impl Iterator<Item = impl Borrow<str>>) -> (usize, usize) {
    let links = parse_all_links(iter);
    let mut stack = vec![];
    (
        get_hashes_of_paths_through_cave(&links, None, &mut stack).len(),
        links
            .keys()
            .filter_map(|elem| {
                if let Node::SmallCave(hash) = elem {
                    Some(get_hashes_of_paths_through_cave(
                        &links,
                        Some(*hash),
                        &mut stack,
                    ))
                } else {
                    None
                }
            })
            .flat_map(|set| set.into_iter())
            .collect::<HashSet<_>>()
            .len(),
    )
}

/// Determines how many paths there are from start to end while allowing the specified small cave to be visited
/// twice (this could be None). This uses a Stack and iteration to avoid blowing the function stack with a recursive method.
/// Stack must store our state tuples, the state being the current node, and the number of times we've visited each small cave.
/// This actually returns the path hashes of the located paths as a set, since part 2 involves us only counting unique paths through the cave
/// with the different small caves allowing double visits.
fn get_hashes_of_paths_through_cave(
    links: &HashMap<Node, HashSet<Node>>,
    allowed_double_visit_hash: Option<u64>,
    stack: &mut impl Stack<(Node, HashMap<u64, u32>, DefaultHasher)>,
) -> HashSet<u64> {
    let mut path_hashes = HashSet::new();
    stack.clear();
    let mut hasher = DefaultHasher::new();
    Node::Start.hash(&mut hasher);
    stack.push((Node::Start, HashMap::new(), hasher));

    while let Some((node, visited, hasher)) = stack.pop() {
        for node in links[&node]
            .iter()
            .filter(|node| is_node_allowed_to_be_visited(node, &visited, allowed_double_visit_hash))
        {
            let mut hasher = hasher.clone();
            node.hash(&mut hasher);
            match node {
                Node::End => { path_hashes.insert(hasher.finish()); },
                Node::LargeCave(_) | Node::SmallCave(_) => {
                    let mut visited = visited.clone();
                    if let Node::SmallCave(hash) = node {
                        *visited.entry(*hash).or_default() += 1;
                    }
                    stack.push(((*node).clone(), visited, hasher));
                },
                _ => panic!("Shouldn't have got here as the start node should be filtered out as a valid transition target.")
            }
        }
    }
    path_hashes
}

/// Function that takes a reference to a Node, the visited counts of small caves, and the allowed double visit hash
/// and determines if it's okay to visit the given Node based on this information.
fn is_node_allowed_to_be_visited(
    node: &Node,
    visited: &HashMap<u64, u32>,
    allowed_double_visit_hash: Option<u64>,
) -> bool {
    match node {
        Node::Start => false,
        Node::End | Node::LargeCave(..) => true,
        Node::SmallCave(hash) => {
            if let Some(count) = visited.get(hash) {
                if let Some(allowed_double_visit_hash) = allowed_double_visit_hash {
                    if *hash == allowed_double_visit_hash {
                        *count < 2 // we're allowed to double visit, but only if we haven't already
                    } else {
                        false // we're not the allowed double visit node, so we've visited once already which is max
                    }
                } else {
                    false // we've visited once before and we're not allowed to double visit any node
                }
            } else {
                true // small caves can be visited at least once
            }
        }
    }
}

/// Function which takes an iterator over elements that can be borrowed as strings, and builds a
/// HashMap of node to a HashSet of the connecting nodes.
fn parse_all_links(iter: impl Iterator<Item = impl Borrow<str>>) -> HashMap<Node, HashSet<Node>> {
    let mut hm = HashMap::new();
    iter.filter_map(|link| {
        let link = link.borrow().trim();
        if link.is_empty() {
            None
        } else {
            Some(parse_link(link))
        }
    })
    .for_each(|link| {
        insert_bidirectional_link(&mut hm, link);
    });
    hm
}

/// Function which takes a mutable reference to a HashMap for storage and a tuple of the two nodes we're
/// linking together, and inserts the link into the hashmap (both directions).
fn insert_bidirectional_link(hm: &mut HashMap<Node, HashSet<Node>>, nodes: (Node, Node)) {
    let (n1, n2) = nodes;
    hm.entry(n1.clone()).or_default().insert(n2.clone());
    hm.entry(n2).or_default().insert(n1);
}

/// Function which takes a link in the input format and returns a pair of nodes that
/// the link joins together.
fn parse_link(string: &str) -> (Node, Node) {
    let mut iter = string.trim().split("-");
    let n1 = iter
        .next()
        .expect(
            "Expected two components separated by a hyphen to form a link, input string invalid.",
        )
        .trim()
        .parse::<Node>()
        .unwrap();
    let n2 = iter
        .next()
        .expect(
            "Expected two components separated by a hyphen to form a link, input string invalid.",
        )
        .trim()
        .parse::<Node>()
        .unwrap();
    (n1, n2)
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        std::{collections::hash_map::DefaultHasher, hash::Hasher},
    };

    #[test]
    #[should_panic]
    fn test_parse_link_empty_string() {
        let _ = parse_link("    ");
    }

    #[test]
    #[should_panic]
    fn test_parse_link_missing_hyphen() {
        let _ = parse_link("blahBLAH");
    }

    #[test]
    fn test_parse_link_success() {
        let mut hasher = DefaultHasher::new();
        hasher.write(b"urhb");
        let hash = hasher.finish();
        let (n1, n2) = parse_link("urhb-end");
        assert_eq!(n1, Node::SmallCave(hash));
        assert_eq!(n2, Node::End);
    }

    #[test]
    fn test_insert_bidirectional_links() {
        let n1 = {
            let mut hasher = DefaultHasher::new();
            hasher.write(b"robin");
            Node::SmallCave(hasher.finish())
        };
        let n2 = {
            let mut hasher = DefaultHasher::new();
            hasher.write(b"PLACE");
            Node::LargeCave(hasher.finish())
        };

        let mut expected = HashMap::new();
        expected
            .entry(Node::Start)
            .or_insert(HashSet::new())
            .insert(Node::End);
        expected
            .entry(Node::End)
            .or_insert(HashSet::new())
            .insert(Node::Start);
        expected
            .entry(n1.clone())
            .or_insert(HashSet::new())
            .insert(Node::Start);
        expected
            .entry(Node::Start)
            .or_insert(HashSet::new())
            .insert(n1.clone());
        expected
            .entry(n1.clone())
            .or_insert(HashSet::new())
            .insert(n2.clone());
        expected
            .entry(n2.clone())
            .or_insert(HashSet::new())
            .insert(n1.clone());
        expected
            .entry(n2.clone())
            .or_insert(HashSet::new())
            .insert(Node::End);
        expected
            .entry(Node::End)
            .or_insert(HashSet::new())
            .insert(n2.clone());

        let mut calculated = HashMap::new();
        insert_bidirectional_link(&mut calculated, (Node::Start, Node::End));
        insert_bidirectional_link(&mut calculated, (Node::Start, n1.clone()));
        insert_bidirectional_link(&mut calculated, (n1, n2.clone()));
        insert_bidirectional_link(&mut calculated, (n2, Node::End));

        assert_eq!(expected, calculated);
    }

    #[test]
    fn test_parse_all_links_empty() {
        assert_eq!(parse_all_links("".lines()), HashMap::new());
    }

    #[test]
    fn test_parse_all_links() {
        let mut expected = HashMap::new();
        let start = Node::Start;
        let end = Node::End;
        let small = {
            let mut hasher = DefaultHasher::new();
            hasher.write(b"robin");
            Node::SmallCave(hasher.finish())
        };
        let large = {
            let mut hasher = DefaultHasher::new();
            hasher.write(b"ROBIN");
            Node::LargeCave(hasher.finish())
        };
        insert_bidirectional_link(&mut expected, (start, small.clone()));
        insert_bidirectional_link(&mut expected, (small, large.clone()));
        insert_bidirectional_link(&mut expected, (large, end));
        let calculated = parse_all_links(
            r#"
        start-robin
        robin-ROBIN
        ROBIN-end
        "#
            .trim()
            .lines(),
        );
        assert_eq!(expected, calculated);
    }

    #[test]
    fn test_find_paths_through_cave_small() {
        assert_eq!(
            count_paths_through_cave(
                r#"
        start-A
        start-b
        A-c
        A-b
        b-d
        A-end
        b-end
        "#
                .trim()
                .lines(),
            ),
            (10, 36)
        );
    }

    #[test]
    fn test_find_paths_through_cave_large() {
        assert_eq!(
            count_paths_through_cave(
                r#"
                dc-end
                HN-start
                start-kj
                dc-start
                dc-HN
                LN-dc
                HN-end
                kj-sa
                kj-HN
                kj-dc
        "#
                .trim()
                .lines(),
            ),
            (19, 103)
        );
    }

    #[test]
    fn test_find_paths_through_cave_supersize() {
        assert_eq!(
            count_paths_through_cave(
                r#"
                fs-end
                he-DX
                fs-he
                start-DX
                pj-DX
                end-zg
                zg-sl
                zg-pj
                pj-he
                RW-he
                fs-DX
                pj-RW
                zg-RW
                start-pj
                he-WI
                zg-he
                pj-fs
                start-RW
        "#
                .trim()
                .lines(),
            ),
            (226, 3509)
        );
    }

    #[test]
    fn test_is_node_allowed_to_be_visited() {
        let mut visited = HashMap::new();
        visited.insert(1, 1);
        visited.insert(2, 2);
        visited.insert(3, 1);

        // test with no double-visits.
        assert!(!is_node_allowed_to_be_visited(&Node::Start, &visited, None));
        assert!(is_node_allowed_to_be_visited(
            &Node::LargeCave(17),
            &visited,
            None
        ));
        assert!(!is_node_allowed_to_be_visited(
            &Node::SmallCave(1),
            &visited,
            None
        ));
        assert!(!is_node_allowed_to_be_visited(
            &Node::SmallCave(2),
            &visited,
            None
        ));
        assert!(!is_node_allowed_to_be_visited(
            &Node::SmallCave(3),
            &visited,
            None
        ));
        assert!(is_node_allowed_to_be_visited(&Node::End, &visited, None));

        // test with double-visit to 1.
        assert!(!is_node_allowed_to_be_visited(
            &Node::Start,
            &visited,
            Some(1)
        ));
        assert!(is_node_allowed_to_be_visited(
            &Node::LargeCave(17),
            &visited,
            Some(1)
        ));
        assert!(is_node_allowed_to_be_visited(
            &Node::SmallCave(1),
            &visited,
            Some(1)
        ));
        assert!(!is_node_allowed_to_be_visited(
            &Node::SmallCave(2),
            &visited,
            Some(1)
        ));
        assert!(!is_node_allowed_to_be_visited(
            &Node::SmallCave(3),
            &visited,
            Some(1)
        ));
        assert!(is_node_allowed_to_be_visited(&Node::End, &visited, Some(1)));

        // test with double-visit to 2.
        assert!(!is_node_allowed_to_be_visited(
            &Node::Start,
            &visited,
            Some(2)
        ));
        assert!(is_node_allowed_to_be_visited(
            &Node::LargeCave(17),
            &visited,
            Some(2)
        ));
        assert!(!is_node_allowed_to_be_visited(
            &Node::SmallCave(1),
            &visited,
            Some(2)
        ));
        assert!(!is_node_allowed_to_be_visited(
            &Node::SmallCave(2),
            &visited,
            Some(2)
        ));
        assert!(!is_node_allowed_to_be_visited(
            &Node::SmallCave(3),
            &visited,
            Some(2)
        ));
        assert!(is_node_allowed_to_be_visited(&Node::End, &visited, Some(2)));

        // test with double-visit to 3.
        assert!(!is_node_allowed_to_be_visited(
            &Node::Start,
            &visited,
            Some(3)
        ));
        assert!(is_node_allowed_to_be_visited(
            &Node::LargeCave(17),
            &visited,
            Some(3)
        ));
        assert!(!is_node_allowed_to_be_visited(
            &Node::SmallCave(1),
            &visited,
            Some(3)
        ));
        assert!(!is_node_allowed_to_be_visited(
            &Node::SmallCave(2),
            &visited,
            Some(3)
        ));
        assert!(is_node_allowed_to_be_visited(
            &Node::SmallCave(3),
            &visited,
            Some(3)
        ));
        assert!(is_node_allowed_to_be_visited(&Node::End, &visited, Some(3)));
    }
}
