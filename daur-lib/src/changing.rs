use crate::time::{Instant, NonZeroInstant};
use std::collections::BTreeMap;

/// A setting that changes over time.
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct Changing<T> {
    /// The starting value.
    pub start: T,
    /// The changes.
    pub changes: BTreeMap<NonZeroInstant, T>,
}

impl<T: Copy> Changing<T> {
    /// Gets the setting at the given instant.
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
