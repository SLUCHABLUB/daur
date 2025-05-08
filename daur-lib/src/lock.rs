use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::fmt;
use std::fmt::{Debug, Formatter};

// TODO: remove
/// A lock.
#[derive(Default)]
pub struct Lock<T> {
    inner: RwLock<T>,
}

impl<T> Lock<T> {
    /// Wraps a value in a lock.
    pub const fn new(value: T) -> Self {
        Lock {
            inner: RwLock::new(value),
        }
    }

    /// Locks the lock for reading.
    pub fn read(&self) -> RwLockReadGuard<T> {
        self.inner.read()
    }

    /// Locks the lock for writing.
    pub fn write(&self) -> RwLockWriteGuard<T> {
        self.inner.write()
    }
}

impl<T: Clone> Clone for Lock<T> {
    fn clone(&self) -> Self {
        Lock::new(self.read().clone())
    }
}

impl<T: Debug> Debug for Lock<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.read().fmt(f)
    }
}
