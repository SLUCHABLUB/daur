use crate::lock::Lock;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

pub struct ArcCell<T: ?Sized> {
    lock: Lock<Arc<T>>,
}

impl<T: ?Sized> ArcCell<T> {
    pub fn new(value: Arc<T>) -> ArcCell<T> {
        ArcCell {
            lock: Lock::new(value),
        }
    }

    pub fn get(&self) -> Arc<T> {
        self.lock.read().clone()
    }

    pub fn set(&self, value: Arc<T>) {
        *self.lock.write() = value;
    }
}

impl<T: Sized> ArcCell<T> {
    pub fn from_value(value: T) -> ArcCell<T> {
        ArcCell::new(Arc::new(value))
    }

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
