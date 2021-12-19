mod number_type;
mod stack_entry;

use {
    crate::common::collections::Stack,
    number_type::{
        contains_number_type, lowest_literal_down_left_branch, lowest_literal_down_right_branch,
        NumberType,
    },
    stack_entry::StackEntry,
    std::{cell::RefCell, convert::Infallible, ops::Add, rc::Rc, str::FromStr},
};

/// Definition of a "Snailfish Number" which is represented as a pair
/// of either other pairs, or regular numbers. Provides methods to reduce the
/// number, get the magnitude, and parse from a string, etc.
#[derive(Debug, PartialEq)]
pub struct Number(Rc<RefCell<NumberType>>); // Number will always have NumberType::Pair at the top level

impl Number {
    /// Reduces the number entirely by repeated application of explode and split.
    /// reduced once both explode and split return false.
    pub fn reduce(&self, stack: &mut impl Stack<StackEntry>) {
        while self.try_explode(stack) || self.try_split(stack) {}
    }

    /// Determines the magnitude of the snailfish number.
    pub fn magnitude(&self) -> u64 {
        self.0.borrow().magnitude()
    }

    /// Navigates the number structure to see if any pair needs to explode. If it does
    /// will perform the explosion and return true. If this function returns false then
    /// no explosion happened (and we should try a split). Uses a preallocated stack to
    /// track the parentage of the explosion. When we're popping off the stack, the left
    /// value will be added to the leftmost regular number of the parent pairs, and the
    /// right value will be added to the rightmost regular number of the parent pairs.
    fn try_explode(&self, stack: &mut impl Stack<StackEntry>) -> bool {
        self.find_explosion(stack);
        if let Some(entry) = stack.pop() {
            let exploded = entry.unwrap();
            self.apply_explosion(stack, &exploded);
            *exploded.borrow_mut() = NumberType::Regular(0); // replace exploded pair with just the number 0
            true
        } else {
            false
        }
    }

    /// Tries to find a regular number in the structure which requires splitting. If it
    /// finds one, it will perform the split and then return true. If this function returns
    /// false then no split happened. Uses a preallocated stack to track what has been explored yet.
    /// More or less the same as explode, except the application is (a LOT) easier as it's just swapping
    /// the element in place from Regular to a pair. The API is different from find_split, since we don't require
    /// the full history kept in the stack like with explode, find_split will wipe the stack and return just the
    /// split entry.
    fn try_split(&self, stack: &mut impl Stack<StackEntry>) -> bool {
        if let Some(split) = self.find_split(stack) {
            let value = split.borrow().extract_regular().unwrap() as f32 / 2.0;
            let floor = NumberType::Regular(value.floor() as u8);
            let ceil = NumberType::Regular(value.ceil() as u8);
            *split.borrow_mut() =
                NumberType::Pair(Rc::new(RefCell::new(floor)), Rc::new(RefCell::new(ceil)));
            true
        } else {
            false
        }
    }

    /// Actually applies the explosion given the remainder of the stack and the left/right values that need
    /// to be exploded. In order to explode, we need to find the "next left" and "next right" regular
    /// numbers. We'd have to do this by moving up the stack one level at a time and traversing down the
    /// left/right branches (ignoring the left/right items themselves) to find out if it has an entry or not.
    /// The next left item is actually traversing the right branch of the parent
    fn apply_explosion(
        &self,
        stack: &mut impl Stack<StackEntry>,
        exploded: &Rc<RefCell<NumberType>>,
    ) {
        // This is safe because we are expecting that the exploded NumberType is a pair (otherwise it wouldn't have exploded).
        let (left, right) = exploded.borrow().decompose_pair().unwrap();

        // Track whether we've already found the "next left" and "next right" entries as we pop off
        // the stack.
        let mut left_target: Option<Rc<RefCell<NumberType>>> = None;
        let mut right_target: Option<Rc<RefCell<NumberType>>> = None;

        // Traverse up the stack to both clear it and also to find the targets to apply
        // the addition to.
        while let Some(entry) = stack.pop() {
            if left_target.is_none() || right_target.is_none() {
                // This is safe because we only ever have a backtrace of pairs on the stack after an explosion.
                let (left, right) = entry.unwrap().borrow().decompose_pair().unwrap();

                // Logic here is that an exploding pair can only have a literal on the left
                // if it's part of a pair that's on the right hand side (and vice versa).
                // Once we have a group on the opposite side to the exploding pair, we can traverse
                // back down it on the closest side to the pair to find the literal target.
                if left_target.is_none() && contains_number_type(&right, exploded) {
                    left_target = Some(lowest_literal_down_right_branch(&left));
                }
                if right_target.is_none() && contains_number_type(&left, exploded) {
                    right_target = Some(lowest_literal_down_left_branch(&right));
                }
            }
        }

        // Apply the addition. We expect that the left/right elements of the exploding pair that are
        // passed in are regular numbers, and we're specifically searching for Regular numbers in the
        // search_for_left_target and search_for_right_target methods, so we can unwrap those too.
        if let Some(left_target) = left_target {
            let total = left_target.borrow().extract_regular().unwrap()
                + left.borrow().extract_regular().unwrap();
            *left_target.borrow_mut() = NumberType::Regular(total);
        }

        if let Some(right_target) = right_target {
            let total = right_target.borrow().extract_regular().unwrap()
                + right.borrow().extract_regular().unwrap();
            *right_target.borrow_mut() = NumberType::Regular(total);
        }
    }

    /// Attempts to find an explosion in the number. Populates the stack with the route to the
    /// exploding pair. If this stack has any items in it, then an explosion happens and the pair
    /// is the
    fn find_explosion(&self, stack: &mut impl Stack<StackEntry>) {
        stack.push(StackEntry::NotTraversed(self.0.clone()));
        while let Some(entry) = stack.pop() {
            match entry {
                StackEntry::NotTraversed(ref ptr) => {
                    if let Some((first, _)) = ptr.clone().borrow().decompose_pair() {
                        if stack.len() == 4 {
                            stack.push(entry);
                            return; // if stack length is 4 after we've pushed our pair off
                        } else {
                            stack.push(StackEntry::TraversedLeft((*ptr).clone()));
                            stack.push(StackEntry::NotTraversed(first));
                        }
                    }
                }
                StackEntry::TraversedLeft(ref ptr) => {
                    if let Some((_, second)) = ptr.borrow().decompose_pair() {
                        stack.push(StackEntry::TraversedRight((*ptr).clone()));
                        stack.push(StackEntry::NotTraversed(second));
                    }
                }
                _ => continue,
            }
        }
    }

    /// Similar to find_explosion except instead of looking for a deeply nested pair, looks for a
    /// literal greater or equal to 10 instead. Additionally, the stack is only needed for tracking
    /// recursion and is wiped out when we return.
    fn find_split(&self, stack: &mut impl Stack<StackEntry>) -> Option<Rc<RefCell<NumberType>>> {
        stack.push(StackEntry::NotTraversed(self.0.clone()));
        while let Some(entry) = stack.pop() {
            match entry {
                StackEntry::NotTraversed(ref ptr) => {
                    if let Some(value) = ptr.borrow().extract_regular() {
                        if value >= 10 {
                            while let Some(_) = stack.pop() {} // drain rest of stack.
                            return Some((*ptr).clone());
                        }
                    } else if let Some((first, _)) = ptr.borrow().decompose_pair() {
                        stack.push(StackEntry::TraversedLeft((*ptr).clone()));
                        stack.push(StackEntry::NotTraversed(first));
                    }
                }
                StackEntry::TraversedLeft(ref ptr) => {
                    if let Some((_, second)) = ptr.borrow().decompose_pair() {
                        stack.push(StackEntry::TraversedRight((*ptr).clone()));
                        stack.push(StackEntry::NotTraversed(second));
                    }
                }
                _ => continue,
            }
        }
        None
    }
}

impl Add for Number {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let num_1 = Rc::try_unwrap(self.0).unwrap().into_inner();
        let num_2 = Rc::try_unwrap(rhs.0).unwrap().into_inner();
        Self(Rc::new(RefCell::new(num_1 + num_2)))
    }
}

// Required because adding two Numbers together does a try_unwrap()/unwrap()
// and the default clone will just clone the root Rc (preventing the unwrap).
impl Clone for Number {
    fn clone(&self) -> Self {
        Self(Rc::new(RefCell::new((*self.0.borrow()).clone())))
    }
}

impl FromStr for Number {
    type Err = Infallible;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        Ok(Self(Rc::new(RefCell::new(
            string.parse::<NumberType>().unwrap(),
        ))))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_explosion_success() {
        let mut stack = Vec::with_capacity(5);
        let number = "[[[[[9,8],1],2],3],4]".parse::<Number>().unwrap();
        number.find_explosion(&mut stack);
        let expected = "[9, 8]".parse::<NumberType>().unwrap();
        assert_eq!(*stack.pop().unwrap().unwrap().borrow(), expected);
    }

    #[test]
    fn test_find_explosion_failure() {
        let mut stack = Vec::with_capacity(5);
        let number = "[[[[9,8],1],2],3]".parse::<Number>().unwrap();
        number.find_explosion(&mut stack);
        assert!(stack.is_empty());
    }

    #[test]
    fn test_try_explode_1() {
        let mut stack = Vec::with_capacity(5);
        let number = "[7,[6,[5,[4,[3,2]]]]]".parse::<Number>().unwrap();
        let expected = "[7,[6,[5,[7,0]]]]".parse::<Number>().unwrap();
        assert!(number.try_explode(&mut stack));
        assert!(stack.is_empty());
        assert_eq!(number, expected);
    }

    #[test]
    fn test_try_explode_2() {
        let mut stack = Vec::with_capacity(5);
        let number = "[[6,[5,[4,[3,2]]]],1]".parse::<Number>().unwrap();
        let expected = "[[6,[5,[7,0]]],3]".parse::<Number>().unwrap();
        assert!(number.try_explode(&mut stack));
        assert!(stack.is_empty());
        assert_eq!(number, expected);
    }

    #[test]
    fn test_try_explode_3() {
        let mut stack = Vec::with_capacity(5);
        let number = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"
            .parse::<Number>()
            .unwrap();
        let expected = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"
            .parse::<Number>()
            .unwrap();
        assert!(number.try_explode(&mut stack));
        assert!(stack.is_empty());
        assert_eq!(number, expected);
    }

    #[test]
    fn test_try_explode_4() {
        let mut stack = Vec::with_capacity(5);
        let number = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"
            .parse::<Number>()
            .unwrap();
        let expected = "[[3,[2,[8,0]]],[9,[5,[7,0]]]]".parse::<Number>().unwrap();
        assert!(number.try_explode(&mut stack));
        assert!(stack.is_empty());
        assert_eq!(number, expected);
    }

    #[test]
    fn test_try_split_1() {
        let mut stack = Vec::with_capacity(5);
        let number = Number(Rc::new(RefCell::new(NumberType::Regular(10))));
        let expected = "[5,5]".parse::<Number>().unwrap();
        assert!(number.try_split(&mut stack));
        assert!(stack.is_empty());
        assert_eq!(number, expected);
    }

    #[test]
    fn test_try_split_2() {
        let mut stack = Vec::with_capacity(5);
        let number = Number(Rc::new(RefCell::new(NumberType::Regular(11))));
        let expected = "[5,6]".parse::<Number>().unwrap();
        assert!(number.try_split(&mut stack));
        assert!(stack.is_empty());
        assert_eq!(number, expected);
    }

    #[test]
    fn test_try_split_3() {
        let mut stack = Vec::with_capacity(5);
        let number = Number(Rc::new(RefCell::new(NumberType::Regular(12))));
        let expected = "[6,6]".parse::<Number>().unwrap();
        assert!(number.try_split(&mut stack));
        assert!(stack.is_empty());
        assert_eq!(number, expected);
    }

    #[test]
    fn test_full_example_1() {
        let first_num = "[[[[4,3],4],4],[7,[[8,4],9]]]".parse::<Number>().unwrap();
        let second_num = "[1,1]".parse::<Number>().unwrap();
        let total = first_num + second_num;
        let mut stack = Vec::with_capacity(5);
        total.reduce(&mut stack);
        let expected = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
            .parse::<Number>()
            .unwrap();
        assert_eq!(total, expected);
    }
}
