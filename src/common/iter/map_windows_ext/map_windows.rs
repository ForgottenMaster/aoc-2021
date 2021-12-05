pub struct MapWindows<T, I, F> {
    wrapped: I,
    func: F,
    window_size: usize,
    window: Vec<T>,
}

impl<T, U, I, F> Iterator for MapWindows<T, I, F>
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

impl<T, I, F> MapWindows<T, I, F> {
    pub fn new(wrapped: I, func: F, window_size: usize) -> Self {
        Self {
            wrapped,
            func,
            window_size,
            window: Vec::with_capacity(window_size),
        }
    }
}
