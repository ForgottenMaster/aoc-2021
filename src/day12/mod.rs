mod node;
mod path;
mod status;

use {
    node::Node,
    path::Path,
    status::Status,
    std::{
        borrow::Borrow,
        collections::{HashMap, HashSet},
        fs::read_to_string,
    },
};

pub fn run() -> (usize, u32) {
    let part_1 = find_paths_through_cave(
        read_to_string("input/day12.txt")
            .expect("Couldn't read input file.")
            .trim()
            .lines(),
    )
    .count();
    (part_1, 0)
}

/// Accepts an iterator over anything that borrows as str (the lines defining the links).
/// Returns an iterator over all paths found through the cave system.
fn find_paths_through_cave(
    iter: impl Iterator<Item = impl Borrow<str>>,
) -> impl Iterator<Item = Path> {
    let links = parse_all_links(iter);
    let mut storage = vec![];
    let mut paths = vec![Some(Path::new())];
    proceed_to_completion(&mut paths, &mut storage, &links);
    paths.into_iter().map(|elem| elem.unwrap())
}

/// Function which takes a mutable vector of Option<Path> and performs all iterations of progressing
/// the paths according to the defined links.
fn proceed_to_completion(
    paths: &mut Vec<Option<Path>>,
    newpath_storage: &mut Vec<Option<Path>>,
    links: &HashMap<Node, HashSet<Node>>,
) {
    loop {
        if let Status::Complete = proceed_paths_once(paths, newpath_storage, links) {
            break;
        }
    }
}

/// Function which takes a mutable vector of Option<Path> and performs one iteration of
/// progressing the path according to the defined links. Removes paths that end in a dead end.
/// Returns a Status indicating whether there are still some incomplete paths or not.
fn proceed_paths_once(
    paths: &mut Vec<Option<Path>>,
    newpath_storage: &mut Vec<Option<Path>>,
    links: &HashMap<Node, HashSet<Node>>,
) -> Status {
    // for each path, we want to take the path out of the option so it can be proceeded
    // and get a replacement set of paths, with which we extend the paths vector.
    for path in paths.into_iter() {
        if let Some(new_paths) = path.take().unwrap().proceed(links) {
            newpath_storage.extend(new_paths.into_iter().map(|elem| Some(elem)));
        }
    }

    // Clear any nones from paths.
    paths.retain(|elem| elem.is_some());

    // Extend with new paths.
    paths.extend(newpath_storage.into_iter().map(|elem| (*elem).clone()));
    newpath_storage.clear();

    // Return appropriate path calculation state.
    if paths
        .iter()
        .all(|elem| elem.as_ref().unwrap().is_complete())
    {
        Status::Complete
    } else {
        Status::Incomplete
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
    fn test_proceed_paths_once() {
        let links = parse_all_links(
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
        );
        let mut paths = vec![Some(Path::new())];
        let mut storage = vec![];
        assert!(matches!(
            proceed_paths_once(&mut paths, &mut storage, &links),
            Status::Incomplete
        ));
        assert_eq!(paths.len(), 2);
    }

    #[test]
    fn test_proceed_to_completion_small() {
        let links = parse_all_links(
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
        );
        let mut paths = vec![Some(Path::new())];
        let mut storage = vec![];
        proceed_to_completion(&mut paths, &mut storage, &links);
        assert_eq!(paths.len(), 10);
    }

    #[test]
    fn test_proceed_to_completion_large() {
        let links = parse_all_links(
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
        );
        let mut paths = vec![Some(Path::new())];
        let mut storage = vec![];
        proceed_to_completion(&mut paths, &mut storage, &links);
        assert_eq!(paths.len(), 19);
    }

    #[test]
    fn test_proceed_to_completion_supersize() {
        let links = parse_all_links(
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
        );
        let mut paths = vec![Some(Path::new())];
        let mut storage = vec![];
        proceed_to_completion(&mut paths, &mut storage, &links);
        assert_eq!(paths.len(), 226);
    }

    #[test]
    fn test_find_paths_through_cave_small() {
        assert_eq!(
            find_paths_through_cave(
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
            )
            .count(),
            10
        );
    }

    #[test]
    fn test_find_paths_through_cave_large() {
        assert_eq!(
            find_paths_through_cave(
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
            )
            .count(),
            19
        );
    }

    #[test]
    fn test_find_paths_through_cave_supersize() {
        assert_eq!(
            find_paths_through_cave(
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
            )
            .count(),
            226
        );
    }
}
