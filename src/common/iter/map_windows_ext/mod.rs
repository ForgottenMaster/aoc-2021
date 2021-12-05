mod map_windows;

use map_windows::MapWindows;

/// Extension trait to add the "map_windows" adapter to any Iterator
/// that supports it.
pub trait MapWindowsExt<T>: Sized {
    fn map_windows<F>(self, window_size: usize, func: F) -> MapWindows<T, Self, F>;
}

impl<T, S> MapWindowsExt<T> for S
where
    S: Sized,
{
    fn map_windows<F>(self, window_size: usize, func: F) -> MapWindows<T, Self, F> {
        MapWindows::new(self, func, window_size)
    }
}
