use parking_lot::{Condvar, Mutex};

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
        let mut guard = self.mutex.lock();

        while !*guard {
            self.condvar.wait(&mut guard);
        }
    }

    pub fn set(&self, value: bool) {
        *self.mutex.lock() = value;
        self.condvar.notify_all();
    }
}
