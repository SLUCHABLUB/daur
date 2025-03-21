use crate::lock::Lock;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

/// A cell maybe containing an [atomically reference counted](Arc) value.
pub struct OptionArcCell<T: ?Sized> {
    lock: Lock<Option<Arc<T>>>,
}

impl<T: ?Sized> OptionArcCell<T> {
    /// Construct a new cell from an optional pointer.
    #[must_use]
    pub const fn new(value: Option<Arc<T>>) -> OptionArcCell<T> {
        OptionArcCell {
            lock: Lock::new(value),
        }
    }

    /// Construct a new cell from a pointer.
    #[must_use]
    pub const fn some(value: Arc<T>) -> OptionArcCell<T> {
        OptionArcCell::new(Some(value))
    }

    /// Constructs a new empty cell.
    #[must_use]
    pub const fn none() -> OptionArcCell<T> {
        OptionArcCell::new(None)
    }

    /// Return a pointer to the optional value.
    #[must_use]
    pub fn get(&self) -> Option<Arc<T>> {
        self.lock.read().clone()
    }

    /// Sets the pointer to a new optional value.
    pub fn set(&self, value: Option<Arc<T>>) {
        *self.lock.write() = value;
    }

    /// Sets the pointer to a new value.
    pub fn set_some(&self, value: Arc<T>) {
        *self.lock.write() = Some(value);
    }
}

impl<T: Sized> OptionArcCell<T> {
    /// Construct a new cell from an optional value.
    #[must_use]
    pub fn from_value(value: Option<T>) -> OptionArcCell<T> {
        OptionArcCell::new(value.map(Arc::new))
    }

    /// Sets the optional value.
    pub fn set_value(&self, value: Option<T>) {
        self.set(value.map(Arc::new));
    }

    /// Sets the optional value.
    pub fn set_some_value(&self, value: T) {
        self.set_some(Arc::new(value));
    }
}

impl<T: ?Sized> Clone for OptionArcCell<T> {
    fn clone(&self) -> Self {
        OptionArcCell {
            lock: Lock::new(self.get()),
        }
    }
}

impl<T: ?Sized + Debug> Debug for OptionArcCell<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.get().fmt(f)
    }
}
