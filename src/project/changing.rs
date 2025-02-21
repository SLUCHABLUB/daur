use crate::time::instant::Instant;
use std::collections::BTreeMap;

#[derive(Clone, Default)]
pub struct Changing<T> {
    pub start: T,
    // TODO: change key to be non-zero
    pub changes: BTreeMap<Instant, T>,
}

impl<T: Copy> Changing<T> {
    pub fn get(&self, instant: Instant) -> T {
        self.changes
            .range(..instant)
            .next_back()
            .map_or(self.start, |(_, value)| *value)
    }
}

impl<T: Copy> From<T> for Changing<T> {
    fn from(start: T) -> Self {
        Changing {
            start,
            changes: BTreeMap::new(),
        }
    }
}
