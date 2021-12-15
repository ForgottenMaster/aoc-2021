use std::{collections::HashMap, iter::once};

pub fn run(input: &str) -> (u32, u32) {
    let (mut polymer_template, insertion_rules) = extract_data(input);
    (0..10)
        .for_each(|_| polymer_template = run_insertion_rules(&polymer_template, &insertion_rules));
    let character_counts = count_characters(&polymer_template);
    let mut sorted_character_counts = character_counts.into_iter().collect::<Vec<_>>();
    sorted_character_counts.sort_by_key(|(_, count)| *count);
    let part_1 =
        sorted_character_counts.last().unwrap().1 - sorted_character_counts.first().unwrap().1;
    (part_1, 0)
}

/// Parses the input string into both a polymer template string and a HashMap containing
/// the insertion rules.
fn extract_data(input: &str) -> (String, HashMap<(char, char), char>) {
    let mut lines = input.trim().lines();
    let polymer_template = lines.next().unwrap().trim().to_string();
    let mut insertion_rules = HashMap::new();
    lines
        .skip_while(|line| line.trim().is_empty())
        .for_each(|line| {
            let mut splits = line.trim().split("->");
            let pair = splits.next().unwrap().trim();
            let first_char = pair.chars().next().unwrap();
            let second_char = pair.chars().skip(1).next().unwrap();
            let inserted_char = splits.next().unwrap().trim().chars().next().unwrap();
            insertion_rules.insert((first_char, second_char), inserted_char);
        });
    (polymer_template, insertion_rules)
}

/// Runs a single rewriting of the insertion rules into the polymer template. All insertions
/// occur simultaneously.
fn run_insertion_rules(input: &str, rules: &HashMap<(char, char), char>) -> String {
    // we can make an "insertion iterator" by making an iterator that is the same length of the input
    // string and pairing each character of the input string up with a character that should come before
    // it in the new string (the inserted character). We'll use '\0' for no insertion and it can be
    // filtered out later. We can use scan in order to create the insertion iterator so we can remember the
    // previous character.
    let insertion_iter = input.chars().scan('\0', |previous, current| {
        let old = *previous;
        *previous = current;
        Some(if let Some(inserted_char) = rules.get(&(old, current)) {
            *inserted_char
        } else {
            '\0'
        })
    });

    // Next, we zip this insertion iterator with the original string, and use a flat map to flatten the stream
    // placing the inserted characters before the original ones. Finally filter to remove any '\0' since they
    // represent the "empty character" and shouldn't appear.
    let character_iter = input
        .chars()
        .zip(insertion_iter)
        .flat_map(|(current, previous)| once(previous).chain(once(current)))
        .filter(|elem| *elem != '\0');

    // Finally we can collect into a new string to return.
    character_iter.collect()
}

/// Counts the characters in the given string and returns a HashMap of character counts.
fn count_characters(input: &str) -> HashMap<char, u32> {
    input.chars().fold(HashMap::new(), |mut hm, elem| {
        *hm.entry(elem).or_default() += 1;
        hm
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_data() {
        const INPUT: &str = r#"
        NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        "#;
        let (polymer_template, insertion_rules) = extract_data(INPUT);
        const EXPECTED_POLYMER: &str = "NNCB";
        let expected_hashmap = {
            let mut hm = HashMap::new();
            hm.insert(('C', 'H'), 'B');
            hm.insert(('H', 'H'), 'N');
            hm.insert(('C', 'B'), 'H');
            hm.insert(('N', 'H'), 'C');
            hm.insert(('H', 'B'), 'C');
            hm
        };
        assert_eq!(polymer_template, EXPECTED_POLYMER);
        assert_eq!(insertion_rules, expected_hashmap);
    }

    #[test]
    fn test_run_insertion_rules() {
        const INPUT: &str = r#"
        NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C
        "#;
        let (polymer_template, insertion_rules) = extract_data(INPUT);
        const AFTER_STEP_1: &str = "NCNBCHB";
        const AFTER_STEP_2: &str = "NBCCNBBBCBHCB";
        const AFTER_STEP_3: &str = "NBBBCNCCNBBNBNBBCHBHHBCHB";
        const AFTER_STEP_4: &str = "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB";
        let polymer_template = run_insertion_rules(&polymer_template, &insertion_rules);
        assert_eq!(polymer_template, AFTER_STEP_1);
        let polymer_template = run_insertion_rules(&polymer_template, &insertion_rules);
        assert_eq!(polymer_template, AFTER_STEP_2);
        let polymer_template = run_insertion_rules(&polymer_template, &insertion_rules);
        assert_eq!(polymer_template, AFTER_STEP_3);
        let polymer_template = run_insertion_rules(&polymer_template, &insertion_rules);
        assert_eq!(polymer_template, AFTER_STEP_4);
    }

    #[test]
    fn test_count_characters() {
        const INPUT: &str = "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB";
        let mut hm = HashMap::new();
        hm.insert('N', 11);
        hm.insert('B', 23);
        hm.insert('C', 10);
        hm.insert('H', 5);
        let calculated = count_characters(INPUT);
        assert_eq!(calculated, hm);
    }
}
