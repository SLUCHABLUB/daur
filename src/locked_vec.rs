use crate::lock::Lock;

pub struct LockedVec<T> {
    inner: Lock<Vec<T>>,
}

impl<T> LockedVec<T> {
    pub const fn new() -> Self {
        LockedVec {
            inner: Lock::new(Vec::new()),
        }
    }
}

impl<T: Clone> Clone for LockedVec<T> {
    fn clone(&self) -> Self {
        LockedVec {
            inner: self.inner.clone(),
        }
    }
}

impl<T> Default for LockedVec<T> {
    fn default() -> Self {
        Self::new()
    }
}
