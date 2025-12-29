use crossbeam::atomic::AtomicCell;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;

/// A thread-safe version of [`Cell`](std::cell::Cell).
#[derive(Default)]
pub struct Cell<T> {
    inner: AtomicCell<T>,
}

impl<T> Cell<T> {
    /// Constructs a new cell.
    #[must_use]
    pub const fn new(value: T) -> Self {
        Cell {
            inner: AtomicCell::new(value),
        }
    }

    /// Replaces the value in the cell.
    pub fn set(&self, value: T) {
        self.inner.store(value);
    }

    /// Replaces the value in the cell.
    pub fn replace(&self, value: T) -> T {
        self.inner.swap(value)
    }
}

impl<T: Copy> Cell<T> {
    /// Copies the value out of the cell.
    #[must_use]
    pub fn get(&self) -> T {
        self.inner.load()
    }
}

impl<T: Default> Cell<T> {
    /// Takes the value out of the cell leaving [`Default::default()`] in its place
    #[must_use]
    pub fn take(&self) -> T {
        self.inner.take()
    }
}

impl<T: Copy> Clone for Cell<T> {
    fn clone(&self) -> Self {
        Cell::new(self.get())
    }
}

impl<T: Copy + PartialEq> PartialEq for Cell<T> {
    fn eq(&self, other: &Self) -> bool {
        self.get() == other.get()
    }
}

impl<T: Copy + Eq> Eq for Cell<T> {}

impl<T: Copy + Debug> Debug for Cell<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.get().fmt(f)
    }
}
