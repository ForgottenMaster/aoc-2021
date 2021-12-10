/// This trait can be used to wrap around a collection such as a Vec and allow pushing/popping
/// from it. We can often use these collections as a stack without a trait (e.g. Push/Pop)
/// but being able to constrain the interface to ONLY allow pushing and popping allows for
/// less chance of errors.
pub trait Stack<T> {
    fn push(&mut self, value: T);
    fn pop(&mut self) -> Option<T>;
    fn clear(&mut self);
    fn len(&self) -> usize;
}

impl<T> Stack<T> for Vec<T> {
    fn push(&mut self, value: T) {
        self.push(value);
    }

    fn pop(&mut self) -> Option<T> {
        self.pop()
    }

    fn clear(&mut self) {
        self.clear();
    }

    fn len(&self) -> usize {
        self.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_push() {
        let mut vec = Vec::new();
        let stack = &mut vec as &mut dyn Stack<u32>;
        stack.push(22u32);
        assert_eq!(vec.len(), 1);
        assert_eq!(vec[0], 22u32);
    }

    #[test]
    fn test_stack_pop() {
        let mut vec = vec![1u32, 2];
        {
            let stack = &mut vec as &mut dyn Stack<u32>;
            assert_eq!(stack.pop().unwrap(), 2);
        }
        assert_eq!(vec.len(), 1);
        assert_eq!(vec[0], 1);
        {
            let stack = &mut vec as &mut dyn Stack<u32>;
            assert_eq!(stack.pop().unwrap(), 1);
            assert!(stack.pop().is_none());
        }
        assert_eq!(vec.len(), 0);
    }

    #[test]
    fn test_stack_clear() {
        let mut vec = vec![1u32, 2];
        let stack = &mut vec as &mut dyn Stack<u32>;
        assert_eq!(stack.len(), 2);
        stack.clear();
        assert_eq!(stack.len(), 0);
    }

    #[test]
    fn test_stack_len() {
        let vec = vec![1u32, 2, 3];
        let stack = &vec as &dyn Stack<u32>;
        assert_eq!(vec.len(), stack.len());
        assert_eq!(stack.len(), 3);
    }
}
