use crate::audio::sample;
use crate::note::Event;
use crate::note::event::Sequence;

/// A sub[sequence](Sequence) of [events](super::Event)
#[derive(Copy, Clone, Debug)]
pub struct Subsequence<'events> {
    events: &'events Sequence,
    period: sample::Period,
}

impl Subsequence<'_> {
    pub(super) fn new(events: &Sequence, period: sample::Period) -> Subsequence<'_> {
        Subsequence { events, period }
    }

    // TODO: take a relative instant
    pub(crate) fn get(&self, timestamp: sample::Instant) -> &[Event] {
        let timestamp = timestamp + self.period.start.since_start;

        if !self.period.contains(timestamp) {
            return &[];
        }

        self.events.get(timestamp)
    }
}
