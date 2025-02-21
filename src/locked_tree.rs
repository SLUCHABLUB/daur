use crate::lock::Lock;
use std::collections::BTreeMap;

#[derive(Clone)]
pub struct LockedTree<K, V> {
    inner: Lock<BTreeMap<K, V>>,
}

impl<K, V> LockedTree<K, V> {
    pub fn new() -> Self {
        LockedTree {
            inner: Lock::new(BTreeMap::new()),
        }
    }
}

impl<K: PartialEq, V: PartialEq> PartialEq for LockedTree<K, V> {
    fn eq(&self, other: &Self) -> bool {
        *self.inner.read() == *other.inner.read()
    }
}

impl<K: Eq, V: Eq> Eq for LockedTree<K, V> {}

impl<K, V> Default for LockedTree<K, V> {
    fn default() -> Self {
        LockedTree::new()
    }
}
