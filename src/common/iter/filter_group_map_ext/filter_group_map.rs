/// Iterator adapter which takes a predicate and filters out those elements that fail
/// but also uses these as a separator for groups. pushes each element that is part of a group
/// into an internal buffer and whenever a separator (element failing the grouping predicate) is encountered
/// will invoke the mapping function with the slice that has accumulated. The mapping function can then do whatever
/// it needs to with the group. We use a mapping function that takes the slice rather than yielding owned vectors as
/// there may be no need to persist the group. If there is then the mapping function can go ahead and do so.
pub struct FilterGroupMap<T, I, F1, F2> {
    group: Vec<T>,
    iterator: I,
    filter_function: F1,
    map_function: F2,
}

impl<T, I, F1, F2> FilterGroupMap<T, I, F1, F2> {
    pub fn new(iterator: I, filter_function: F1, map_function: F2) -> Self {
        Self {
            group: vec![],
            iterator,
            filter_function,
            map_function,
        }
    }
}

/// Iterator implementation for FilterGroupMap. Will yield the result of applying the map function to the groups
/// determined by the filter_function.
impl<I, F1, F2, U> Iterator for FilterGroupMap<I::Item, I, F1, F2>
where
    I: Iterator,
    F1: Fn(&I::Item) -> bool,
    F2: Fn(&[I::Item]) -> U,
{
    type Item = U;

    fn next(&mut self) -> Option<Self::Item> {
        let mut emit_group = false;
        loop {
            if emit_group {
                let result = (self.map_function)(&self.group);
                self.group.clear();
                break Some(result);
            }

            if let Some(elem) = self.iterator.next() {
                if (self.filter_function)(&elem) {
                    self.group.push(elem);
                } else if !self.group.is_empty() {
                    emit_group = true;
                }
            } else if !self.group.is_empty() {
                emit_group = true;
            } else {
                break None;
            }
        }
    }
}
