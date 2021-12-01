use std::{
    fs::File,
    io::{BufRead, BufReader, Result, Seek},
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////

fn main() -> Result<()> {
    let file = File::open("input/day01.txt")?;
    let mut reader = BufReader::new(file);
    println!("Part 1 => {}", run(&mut reader, 1));
    reader.rewind()?;
    println!("Part 2 => {}", run(&mut reader, 3));
    Ok(())
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq)]
enum Delta {
    Unchanged,
    Increased,
    Decreased,
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Custom Iterator that will create sliding windows of a specified length along
/// the wrapped iterator.
struct Windows<T, I, F> {
    wrapped: I,
    func: F,
    window_size: usize,
    window: Vec<T>,
}

impl<T, U, I, F> Iterator for Windows<T, I, F>
where
    I: Iterator<Item = T>,
    F: Fn(&[T]) -> U,
{
    type Item = U;

    fn next(&mut self) -> Option<Self::Item> {
        if self.window.len() == 0 {
            // window isn't populated yet, add elements from the underlying
            // iterator until we have self.window_size in the queue (or run out).
            loop {
                if self.window.len() == self.window_size {
                    break Some((self.func)(&self.window));
                } else if let Some(next) = self.wrapped.next() {
                    self.window.push(next);
                } else {
                    break None;
                }
            }
        } else {
            // window was initially populated, simply need to remove the first element
            // and push the new element in.
            if let Some(next) = self.wrapped.next() {
                self.window.remove(0);
                self.window.push(next);
                Some((self.func)(&self.window))
            } else {
                None
            }
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Extension trait to add the "with_windows" adapter to any Iterator
/// that supports it.
trait WithWindows<T, U>: Iterator<Item = T> + Sized {
    fn with_windows<F: Fn(&[T]) -> U>(self, window_size: usize, func: F) -> Windows<T, Self, F>;
}

impl<T, U, S> WithWindows<T, U> for S
where
    S: Iterator<Item = T> + Sized,
{
    fn with_windows<F: Fn(&[T]) -> U>(self, window_size: usize, func: F) -> Windows<T, Self, F> {
        Windows {
            wrapped: self,
            func,
            window_size,
            window: Vec::with_capacity(window_size),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Processes the input to count how many depth increases there are in the sums
/// of a given sliding window size (part 1 = 1, part 2 = 3).
fn run(reader: impl BufRead, window_size: usize) -> usize {
    reader
        .lines() // iterator over the lines of the reader
        .filter(|elem| elem.is_ok()) // lines elements can be errors, only take the ones that read correctly
        .map(|elem| elem.unwrap().trim().parse::<u32>()) // unwrap the valid results and parse as a u32
        .filter(|elem| elem.is_ok()) // again, the u32 parse might fail due to the line being an invalid number or something, filter out invalid
        .map(|elem| elem.unwrap()) // unwrap the now valid only results to get an Iterator<Item=u32>
        .with_windows(window_size, |elems: &[u32]| {
            Some(elems.into_iter().sum::<u32>())
        }) // use the sliding windows, and sum each window to get a new iterator of values
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

///////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn test_example_part_1() {
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
        let calculated = run(INPUT, 1);
        assert_eq!(calculated, EXPECTED);
    }

    #[test]
    fn test_example_part_2() {
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
        const EXPECTED: usize = 5;
        let calculated = run(INPUT, 3);
        assert_eq!(calculated, EXPECTED);
    }
}
