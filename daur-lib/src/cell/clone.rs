use crate::lock::Lock;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::mem::replace;

/// A cell containing a clonable value.
#[derive(Clone, Default)]
pub struct CloneCell<T> {
    lock: Lock<T>,
}

impl<T: Clone> CloneCell<T> {
    /// Construct a new cell from a pointer.
    pub const fn new(value: T) -> CloneCell<T> {
        CloneCell {
            lock: Lock::new(value),
        }
    }

    /// Return a pointer to the value.
    pub fn get(&self) -> T {
        self.lock.read().clone()
    }

    /// Sets the pointer to a new value.
    pub fn set(&self, value: T) {
        *self.lock.write() = value;
    }

    pub(crate) fn replace(&self, value: T) -> T {
        replace(&mut self.lock.write(), value)
    }

    pub(super) fn lock_ref(&self) -> &Lock<T> {
        &self.lock
    }
}

impl<T: Clone + Debug> Debug for CloneCell<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.get().fmt(f)
    }
}
