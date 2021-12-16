use {crate::common::iter::MapWindowsExt, std::collections::HashMap};

pub fn run(input: &str) -> (u64, u64) {
    let (polymer_template, insertion_rules) = extract_data(input);
    let last_char = polymer_template.chars().rev().next().unwrap();
    let mut pair_frequencies = generate_pair_mapping(&polymer_template);
    (0..10).for_each(|_| {
        pair_frequencies = apply_rules(&pair_frequencies, &insertion_rules);
    });
    let part_1 = {
        get_difference_between_most_and_least_common(&mut count_letter_frequencies(
            &pair_frequencies,
            last_char,
        ))
    };
    (0..30).for_each(|_| {
        pair_frequencies = apply_rules(&pair_frequencies, &insertion_rules);
    });
    let part_2 = {
        get_difference_between_most_and_least_common(&mut count_letter_frequencies(
            &pair_frequencies,
            last_char,
        ))
    };
    (part_1, part_2)
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

/// Runs through each window of size 2 in the string and counts the number of each pair
/// into a HashMap.
fn generate_pair_mapping(input: &str) -> HashMap<(char, char), u64> {
    let mut hm = HashMap::with_capacity(input.len());
    input
        .chars()
        .map_windows(2, |window: &[char]| (window[0], window[1]))
        .for_each(|tuple| {
            *hm.entry(tuple).or_default() += 1;
        });
    hm
}

/// Applies the rules to given pair mapping, this simply runs through each pair and
/// knows that the insertion produces equal amounts of 2 pairs (equal to the count of the
/// original pair).
fn apply_rules(
    pair_frequencies: &HashMap<(char, char), u64>,
    rules: &HashMap<(char, char), char>,
) -> HashMap<(char, char), u64> {
    let mut new_frequencies = HashMap::with_capacity(rules.len());
    pair_frequencies.into_iter().for_each(|(tuple, count)| {
        let char_1 = tuple.0;
        let char_2 = *rules.get(&tuple).unwrap(); // assume the rules are complete.
        let char_3 = tuple.1;
        *new_frequencies.entry((char_1, char_2)).or_default() += count;
        *new_frequencies.entry((char_2, char_3)).or_default() += count;
    });
    new_frequencies
}

/// When we count the letter frequencies, we'll only take the first letter of each pair to count
/// since each letter will be in 2 pairs and shouldn't be counted twice. The very last letter of the polymer
/// however isn't counted at all in that case, but because insertions always push the last letter to the end
/// we can just factor that in.
fn count_letter_frequencies(
    pair_frequencies: &HashMap<(char, char), u64>,
    final_char: char,
) -> Vec<(char, u64)> {
    let mut letter_frequencies = HashMap::with_capacity(pair_frequencies.len()); // better to overestimate capacity and only allocate once.
    pair_frequencies
        .into_iter()
        .for_each(|((character, _), count)| {
            *letter_frequencies.entry(*character).or_default() += count;
        });
    *letter_frequencies.entry(final_char).or_default() += 1;
    letter_frequencies.into_iter().collect()
}

/// Function that takes the list of letter frequencies and sorts them, before calculating the difference between the
/// most common (last) and least common (first) letters.
fn get_difference_between_most_and_least_common(letter_frequencies: &mut [(char, u64)]) -> u64 {
    letter_frequencies.sort_by_key(|(_, count)| *count);
    letter_frequencies.last().unwrap().1 - letter_frequencies.first().unwrap().1
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
    fn test_generate_pair_mapping() {
        const INPUT: &str = "NBCCNBBBCBHCB";
        let mut hm = HashMap::new();
        hm.insert(('N', 'B'), 2);
        hm.insert(('B', 'C'), 2);
        hm.insert(('C', 'C'), 1);
        hm.insert(('C', 'N'), 1);
        hm.insert(('B', 'B'), 2);
        hm.insert(('C', 'B'), 2);
        hm.insert(('B', 'H'), 1);
        hm.insert(('H', 'C'), 1);
        let calculated = generate_pair_mapping(INPUT);
        assert_eq!(calculated, hm);
    }

    #[test]
    fn test_apply_rules() {
        let mut hm_in = HashMap::with_capacity(3);
        hm_in.insert(('N', 'N'), 1);
        hm_in.insert(('N', 'C'), 1);
        hm_in.insert(('C', 'B'), 1);

        let mut hm_out = HashMap::with_capacity(6);
        hm_out.insert(('N', 'C'), 1);
        hm_out.insert(('C', 'N'), 1);
        hm_out.insert(('N', 'B'), 1);
        hm_out.insert(('B', 'C'), 1);
        hm_out.insert(('C', 'H'), 1);
        hm_out.insert(('H', 'B'), 1);

        let mut hm_rules = HashMap::with_capacity(3);
        hm_rules.insert(('N', 'N'), 'C');
        hm_rules.insert(('N', 'C'), 'B');
        hm_rules.insert(('C', 'B'), 'H');

        let calculated = apply_rules(&hm_in, &hm_rules);
        assert_eq!(calculated, hm_out);
    }

    #[test]
    fn test_count_letter_frequencies() {
        let final_char = 'B';
        let mut hm_in = HashMap::with_capacity(6);
        hm_in.insert(('N', 'C'), 1);
        hm_in.insert(('C', 'N'), 1);
        hm_in.insert(('N', 'B'), 1);
        hm_in.insert(('B', 'C'), 1);
        hm_in.insert(('C', 'H'), 1);
        hm_in.insert(('H', 'B'), 1);

        let mut letter_frequencies = Vec::with_capacity(4);
        letter_frequencies.push(('N', 2));
        letter_frequencies.push(('C', 2));
        letter_frequencies.push(('B', 2));
        letter_frequencies.push(('H', 1));

        let calculated = count_letter_frequencies(&hm_in, final_char);

        // Can't know what order the hashmap returns the keys, so for testing
        // we convert back to HashMaps
        assert_eq!(
            calculated.into_iter().collect::<HashMap<_, _>>(),
            letter_frequencies.into_iter().collect::<HashMap<_, _>>()
        );
    }

    #[test]
    fn test_get_difference_between_most_common_and_least_common() {
        let mut letter_frequencies = vec![('N', 2), ('H', 1), ('B', 2), ('C', 2)];
        let calculated = get_difference_between_most_and_least_common(&mut letter_frequencies);
        assert_eq!(calculated, 1);
    }
}
