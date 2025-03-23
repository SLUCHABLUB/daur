#![expect(clippy::mutex_atomic, reason = "condvar needs a mutex guard")]

use std::sync::{Condvar, Mutex, PoisonError};

pub struct Condition {
    mutex: Mutex<bool>,
    condvar: Condvar,
}

impl Condition {
    pub fn new(initial_state: bool) -> Condition {
        Condition {
            mutex: Mutex::new(initial_state),
            condvar: Condvar::new(),
        }
    }

    /// Waits until the condition becomes true.
    pub fn wait_until(&self) {
        let mut guard = self.mutex.lock().unwrap_or_else(PoisonError::into_inner);

        while !*guard {
            guard = self
                .condvar
                .wait(guard)
                .unwrap_or_else(PoisonError::into_inner);
        }
    }

    pub fn set(&self, value: bool) {
        *self.mutex.lock().unwrap_or_else(PoisonError::into_inner) = value;
        self.condvar.notify_all();
    }
}
