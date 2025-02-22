use crate::time::{Instant, NonZeroInstant};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default)]
pub struct Changing<T> {
    pub start: T,
    pub changes: BTreeMap<NonZeroInstant, T>,
}

impl<T: Copy> Changing<T> {
    pub fn get(&self, instant: Instant) -> T {
        let Some(end) = NonZeroInstant::from_instant(instant) else {
            return self.start;
        };

        self.changes
            .range(..end)
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
