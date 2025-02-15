use crate::lock::Lock;
use std::sync::{Arc, Weak};
use std::vec::IntoIter;

pub struct LockedVec<T> {
    inner: Lock<Vec<T>>,
}

impl<T> LockedVec<T> {
    pub const fn new() -> Self {
        LockedVec {
            inner: Lock::new(Vec::new()),
        }
    }

    pub fn len(&self) -> usize {
        self.inner.read().len()
    }

    pub fn push(&self, element: T) -> usize {
        let mut vec = self.inner.write();
        let index = vec.len();
        vec.push(element);
        index
    }

    pub fn update<R>(&self, index: usize, f: impl FnOnce(&mut T) -> R) -> Option<R> {
        self.inner.write().get_mut(index).map(f)
    }

    pub fn map<R>(&self, mut f: impl FnMut(&T) -> R) -> IntoIter<R> {
        let vec = self.inner.read();

        let mut result = Vec::with_capacity(vec.len());

        for element in vec.as_slice() {
            result.push(f(element));
        }

        result.into_iter()
    }

    pub fn map_enumerated<R>(&self, mut f: impl FnMut(usize, &T) -> R) -> IntoIter<R> {
        let mut index = 0;
        self.map(|element| {
            let element = f(index, element);
            index += 1;
            element
        })
    }
}

impl<T> LockedVec<Arc<T>> {
    pub fn remove(&self, weak: &Weak<T>)
    where
        T: Eq,
    {
        let mut vec = self.inner.write();

        let Some(target) = weak.upgrade() else {
            return;
        };

        let Some(index) = vec.iter().position(|arc| *arc == target) else {
            return;
        };

        vec.remove(index);
    }

    pub fn iter(&self) -> IntoIter<Arc<T>> {
        self.map(Arc::clone)
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
