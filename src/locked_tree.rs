use crate::lock::Lock;
use itertools::Itertools;
use std::collections::BTreeMap;
use std::ops::Bound;
use std::vec::IntoIter;

#[derive(Default)]
pub struct LockedTree<K, V> {
    inner: Lock<BTreeMap<K, V>>,
}

impl<K, V> LockedTree<K, V> {
    pub fn new() -> Self {
        LockedTree {
            inner: Lock::new(BTreeMap::new()),
        }
    }

    pub fn map<R>(&self, mut f: impl FnMut(&K, &V) -> R) -> IntoIter<R> {
        let map = self.inner.read();
        let mut result = Vec::new();

        for (key, value) in map.iter() {
            result.push(f(key, value));
        }

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
        self.inner.read().range(..key).next_back().map(|(_, v)| *v)
    }

    pub fn iter_gt(&self, start: K) -> IntoIter<(K, V)> {
        let range = (Bound::Excluded(start), Bound::Unbounded);
        self.inner
            .read()
            .range(range)
            .map(|(k, v)| (*k, *v))
            .collect_vec()
            .into_iter()
    }
}

impl<K: Clone, V: Clone> Clone for LockedTree<K, V> {
    fn clone(&self) -> Self {
        LockedTree {
            inner: self.inner.clone(),
        }
    }
}
