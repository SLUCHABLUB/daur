//! Items pertaining to [`ArcCell`].

use parking_lot::Mutex;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::sync::Arc;

/// A cell containing an [atomically reference counted](Arc) value.
pub struct ArcCell<T: ?Sized> {
    /// The underlying lock.
    lock: Mutex<Arc<T>>,
}

impl<T: ?Sized> ArcCell<T> {
    /// Construct a new cell from a pointer.
    pub const fn new(value: Arc<T>) -> ArcCell<T> {
        ArcCell {
            lock: Mutex::new(value),
        }
    }

    /// Return a pointer to the value.
    pub fn get(&self) -> Arc<T> {
        self.lock.lock().clone()
    }

    /// Sets the pointer to a new value.
    pub fn set(&self, value: Arc<T>) {
        *self.lock.lock() = value;
    }
}

impl<T: Debug + ?Sized> Debug for ArcCell<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.get().fmt(f)
    }
}
