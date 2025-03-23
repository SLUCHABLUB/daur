use parking_lot::{Condvar, Mutex};
use std::fmt;
use std::fmt::{Debug, Formatter};

/// An internally mutable value which can be waited for.
pub struct Observed<T> {
    mutex: Mutex<T>,
    condvar: Condvar,
}

impl<T> Observed<T> {
    /// Wraps a value.
    pub const fn new(value: T) -> Observed<T> {
        Observed {
            mutex: Mutex::new(value),
            condvar: Condvar::new(),
        }
    }

    /// Sets the internal value, notifying all observers.
    pub fn set(&self, value: T) {
        *self.mutex.lock() = value;
        self.condvar.notify_all();
    }
}

impl<T: Copy> Observed<T> {
    /// Copies the internal value.
    pub fn get(&self) -> T {
        *self.mutex.lock()
    }
}

impl<T: Eq> Observed<T> {
    /// Waits while the condition returns true.
    pub fn wait_while<Condition>(&self, condition: Condition)
    where
        Condition: FnMut(&mut T) -> bool,
    {
        let mut guard = self.mutex.lock();
        self.condvar.wait_while(&mut guard, condition);
    }

    /// Waits until the internal value becomes a certain value.
    pub fn wait_for(&self, value: &T) {
        self.wait_while(|new| new != value);
    }
}

impl Observed<bool> {
    /// Waits until the internal value becomes true.
    pub fn wait_until(&self) {
        self.wait_for(&true);
    }
}

impl<T: Debug> Debug for Observed<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.mutex.lock().fmt(f)
    }
}
