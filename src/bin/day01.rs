use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
};

fn main() -> Result<()> {
    let file = File::open("input/day01.txt")?;
    let reader = BufReader::new(file);
    println!("Part 1 => {}", run(reader));
    Ok(())
}

#[derive(PartialEq)]
enum Delta {
    Unchanged,
    Increased,
    Decreased,
}

fn run(reader: impl BufRead) -> usize {
    reader
        .lines() // iterator over the lines of the reader
        .filter(|elem| elem.is_ok()) // lines elements can be errors, only take the ones that read correctly
        .map(|elem| elem.unwrap().trim().parse::<u32>()) // unwrap the valid results and parse as a u32
        .filter(|elem| elem.is_ok()) // again, the u32 parse might fail due to the line being an invalid number or something, filter out invalid
        .map(|elem| elem.unwrap()) // unwrap the now valid only results to get an Iterator<Item=u32>
        .scan(None, |state, elem| {
            let return_value = match (state.as_ref(), elem) {
                (Some(&state), elem) if elem > state => Delta::Increased,
                (Some(&state), elem) if elem < state => Delta::Decreased,
                _ => Delta::Unchanged,
            };
            *state = Some(elem); // update state for next element to compare against
            Some(return_value)
        }) // scan through the iterator, comparing each element against previous (recorded in the mutable state)
        .filter(|elem| *elem == Delta::Increased) // only interested in increases
        .count() // final count of increases
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn test_example() {
        const INPUT: &[u8] = r#"
        199
        200
        208
        210
        200
        207
        240
        269
        260
        263
        "#
        .as_bytes();
        const EXPECTED: usize = 7;
        let calculated = run(INPUT);
        assert_eq!(calculated, EXPECTED);
    }
}
