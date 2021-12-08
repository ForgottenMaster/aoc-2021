use std::{fmt::Display, fs::read_to_string};

pub fn run() -> (impl Display, impl Display) {
    let file_content = read_to_string("input/day08.txt").expect("Could not read contents of file.");
    let part_1 = calculate_part_1(&file_content);
    (part_1, 0)
}

fn calculate_part_1(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .split("|")
                .skip(1)
                .next()
                .expect("Could not find separator '|' on one of the input lines.")
                .trim()
                .split(" ")
                .filter(|elem| {
                    let len = elem.trim().len();
                    len == 2 || len == 4 || len == 3 || len == 7
                })
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        const INPUT: &str = r#"
        be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
        "#;
        const EXPECTED: usize = 26;
        let calculated = calculate_part_1(&INPUT);
        assert_eq!(calculated, EXPECTED);
    }
}
