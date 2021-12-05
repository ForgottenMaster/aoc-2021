mod filter_group_map;

use filter_group_map::FilterGroupMap;

/// Extension trait which allows us to call filter_map_group on any iterator and produce a decorated iterator that
/// performs the filter, group, and map functionality.
pub trait FilterGroupMapExt<T, I, F1, F2> {
    fn filter_group_map(
        self,
        filter_function: F1,
        map_function: F2,
    ) -> FilterGroupMap<T, I, F1, F2>;
}

/// Blanket implementation of FilterGroupMapExt for all compatible iterators.
impl<I, F1, F2, U> FilterGroupMapExt<I::Item, I, F1, F2> for I
where
    I: Iterator,
    F1: Fn(&I::Item) -> bool,
    F2: Fn(&[I::Item]) -> U,
{
    fn filter_group_map(
        self,
        filter_function: F1,
        map_function: F2,
    ) -> FilterGroupMap<<Self as Iterator>::Item, Self, F1, F2> {
        FilterGroupMap::new(self, filter_function, map_function)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_group_empty() {
        const INPUT: &str = r#"
        


        "#;
        let mut iter = INPUT
            .lines()
            .filter_group_map(|line| !line.trim().is_empty(), |group| group.to_owned());
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_filter_group_single_group_multi_line() {
        const INPUT: &str = r#"
        line 1
        line 2
        "#;
        let mut iter = INPUT
            .lines()
            .map(|line| line.trim())
            .filter_group_map(|line| !line.trim().is_empty(), |group| group.to_owned());
        assert_eq!(iter.next().unwrap(), vec!["line 1", "line 2"]);
    }

    #[test]
    fn test_filter_group_multi_group() {
        const INPUT: &str = r#"
        line 1

        line 1
        line 2


        line 1

        line 1
        line 2
        line 3
        "#;
        let mut iter = INPUT
            .lines()
            .map(|line| line.trim())
            .filter_group_map(|line| !line.trim().is_empty(), |group| group.to_owned());
        assert_eq!(iter.next().unwrap(), vec!["line 1"]);
        assert_eq!(iter.next().unwrap(), vec!["line 1", "line 2"]);
        assert_eq!(iter.next().unwrap(), vec!["line 1"]);
        assert_eq!(iter.next().unwrap(), vec!["line 1", "line 2", "line 3"]);
    }
}
