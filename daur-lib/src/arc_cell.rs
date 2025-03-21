use crate::lock::Lock;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

/// A cell containing an [atomically reference counted](Arc) value.
pub struct ArcCell<T: ?Sized> {
    lock: Lock<Arc<T>>,
}

impl<T: ?Sized> ArcCell<T> {
    /// Construct a new cell from a pointer.
    pub fn new(value: Arc<T>) -> ArcCell<T> {
        ArcCell {
            lock: Lock::new(value),
        }
    }

    /// Return a pointer to the value.
    pub fn get(&self) -> Arc<T> {
        self.lock.read().clone()
    }

    /// Sets the pointer to a new value.
    pub fn set(&self, value: Arc<T>) {
        *self.lock.write() = value;
    }
}

impl<T: Sized> ArcCell<T> {
    /// Construct a new cell from a value.
    pub fn from_value(value: T) -> ArcCell<T> {
        ArcCell::new(Arc::new(value))
    }

    /// Sets the value.
    pub fn set_value(&self, value: T) {
        self.set(Arc::new(value));
    }
}

impl<T: ?Sized> Clone for ArcCell<T> {
    fn clone(&self) -> Self {
        ArcCell {
            lock: Lock::new(self.get()),
        }
    }
}

impl<T: ?Sized + Debug> Debug for ArcCell<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.get().fmt(f)
    }
}
