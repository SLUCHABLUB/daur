use crate::time::instant::Instant;
use std::collections::BTreeMap;
use std::ops::Index;

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct Changing<T> {
    pub start: T,
    pub changes: BTreeMap<Instant, T>,
}

impl<T> Index<Instant> for Changing<T> {
    type Output = T;

    fn index(&self, instant: Instant) -> &Self::Output {
        self.changes
            .range(..instant)
            .next_back()
            .map_or(&self.start, |(_, value)| value)
    }
}

impl<T> From<T> for Changing<T> {
    fn from(start: T) -> Self {
        Changing {
            start,
            changes: BTreeMap::new(),
        }
    }
}
