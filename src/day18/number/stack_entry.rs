use {
    super::NumberType,
    std::{cell::RefCell, rc::Rc},
};

/// We use a stack when we're finding exploding pairs as we need to find the
/// leftmost pair and so traverse down the left branch first, then the right
/// branch. However if we push raw NumberType instances into the stack we lose
/// information on whether we have traversed the left or right side (or neither).
/// We therefore push StackEntries on instead which indicate the state of traversal
/// which can either be NotTraversed, TraversedLeft, or TraversedRight. This allows us
/// to ensure the correct NumberType entry in the pair is pushed onto the stack in the correct
/// order.
///
/// We need to use Rc in order to be able to pass pointers around etc without having to clone
/// the underlying data. Passing a Box would require a clone or move as Box owns the data.
///
/// Since we'll have multiple Rc pointers to the NumberType though, we can't mutate through
/// Rc::get_mut and will actually need interior mutability. Hence the RefCell.
#[derive(Debug, PartialEq)]
pub enum StackEntry {
    NotTraversed(Rc<RefCell<NumberType>>),
    TraversedLeft(Rc<RefCell<NumberType>>),
    TraversedRight(Rc<RefCell<NumberType>>),
}

impl StackEntry {
    /// Unwraps the entry and returns the contained reference counted NumberType.
    pub fn unwrap(self) -> Rc<RefCell<NumberType>> {
        match self {
            Self::NotTraversed(ret) => ret,
            Self::TraversedLeft(ret) => ret,
            Self::TraversedRight(ret) => ret,
        }
    }
}
