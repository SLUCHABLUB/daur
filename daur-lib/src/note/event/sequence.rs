use crate::audio::sample::{Instant, Period};
use crate::note::Event;
use crate::note::event::Subsequence;
use mitsein::vec1::Vec1;
use std::collections::BTreeMap;
use std::collections::btree_map::Entry;

/// A sequence of events sorted by their timestamp.
#[derive(Clone, Debug, Default)]
pub struct Sequence {
    events: BTreeMap<Instant, Vec1<Event>>,
}

impl Sequence {
    pub const fn new() -> Sequence {
        Sequence {
            events: BTreeMap::new(),
        }
    }

    pub(crate) fn subsequence(&self, period: Period) -> Subsequence {
        Subsequence::new(self, period)
    }

    pub(crate) fn get(&self, timestamp: Instant) -> &[Event] {
        self.events.get(&timestamp).map_or(&[], Vec1::as_ref)
    }

    pub(crate) fn insert(&mut self, timestamp: Instant, event: Event) {
        match self.events.entry(timestamp) {
            Entry::Vacant(entry) => {
                entry.insert(Vec1::from_one(event));
            }
            Entry::Occupied(mut entry) => entry.get_mut().push(event),
        }
    }

    pub(crate) fn into_iterator(self) -> impl Iterator<Item = (Instant, Event)> {
        self.events
            .into_iter()
            .flat_map(|(timestamp, events)| events.into_iter().map(move |event| (timestamp, event)))
    }

    pub(crate) fn last_timestamp(&self) -> Option<Instant> {
        self.events.keys().next_back().copied()
    }
}

impl Extend<(Instant, Event)> for Sequence {
    fn extend<T: IntoIterator<Item = (Instant, Event)>>(&mut self, iter: T) {
        for (timestamp, event) in iter {
            self.insert(timestamp, event);
        }
    }
}

impl FromIterator<(Instant, Event)> for Sequence {
    fn from_iter<T: IntoIterator<Item = (Instant, Event)>>(iter: T) -> Self {
        let mut events = Sequence::default();

        events.extend(iter);

        events
    }
}
