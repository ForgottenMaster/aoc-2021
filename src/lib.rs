/// Custom Iterator that will create sliding windows of a specified length along
/// the wrapped iterator.
pub struct Windows<T, I, F> {
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

/// Extension trait to add the "with_windows" adapter to any Iterator
/// that supports it.
pub trait WithWindows<T, U>: Iterator<Item = T> + Sized {
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
