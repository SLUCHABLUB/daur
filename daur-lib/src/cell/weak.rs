use crate::lock::Lock;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::sync::Weak;

/// A cell containing a [weak atomically reference counted](Weak) value.
pub struct WeakCell<T: ?Sized> {
    lock: Lock<Weak<T>>,
}

impl<T: ?Sized> WeakCell<T> {
    /// Construct a new cell from a pointer.
    pub fn new(value: Weak<T>) -> WeakCell<T> {
        WeakCell {
            lock: Lock::new(value),
        }
    }

    /// Return a pointer to the value.
    pub fn get(&self) -> Weak<T> {
        self.lock.read().clone()
    }

    /// Sets the pointer to a new value.
    pub fn set(&self, value: Weak<T>) {
        *self.lock.write() = value;
    }
}

impl<T: ?Sized> Clone for WeakCell<T> {
    fn clone(&self) -> Self {
        WeakCell {
            lock: Lock::new(self.get()),
        }
    }
}

impl<T: ?Sized + Debug> Debug for WeakCell<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.get().fmt(f)
    }
}
