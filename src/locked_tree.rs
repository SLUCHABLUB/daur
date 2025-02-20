use crate::lock::Lock;
use itertools::Itertools as _;
use std::collections::BTreeMap;
use std::ops::Bound;
use std::vec::IntoIter;

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

    pub fn for_each<F: FnMut(&K, &V)>(&self, mut f: F) {
        let map = self.inner.read();

        for (key, value) in map.iter() {
            f(key, value);
        }
    }

    pub fn map<R, F: FnMut(&K, &V) -> R>(&self, mut f: F) -> IntoIter<R> {
        let mut result = Vec::new();

        self.for_each(|key, value| {
            result.push(f(key, value));
        });

        result.into_iter()
    }
}

impl<K: Ord, V> LockedTree<K, V> {
    pub fn insert(&self, key: K, value: V) -> Option<V> {
        self.inner.write().insert(key, value)
    }
}

impl<K: Copy + Ord, V: Copy> LockedTree<K, V> {
    pub fn get_lte(&self, key: K) -> Option<V> {
        self.inner
            .read()
            .range(..key)
            .next_back()
            .map(|(_, value)| *value)
    }

    pub fn iter_gt(&self, start: K) -> IntoIter<(K, V)> {
        let range = (Bound::Excluded(start), Bound::Unbounded);
        self.inner
            .read()
            .range(range)
            .map(|(key, value)| (*key, *value))
            .collect_vec()
            .into_iter()
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
