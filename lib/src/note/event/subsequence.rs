//! Items pertaining to [`Subsequence`].

use crate::audio::sample;
use crate::note::Event;
use crate::note::event::Sequence;

/// A sub[sequence](Sequence) of [events](super::Event).
#[derive(Copy, Clone, Debug)]
pub struct Subsequence<'events> {
    /// The super-sequence.
    events: &'events Sequence,
    /// The period of the subsequence.
    period: sample::Period,
}

impl Subsequence<'_> {
    /// Constructs a new subsequence.
    pub(super) fn new(events: &Sequence, period: sample::Period) -> Subsequence<'_> {
        Subsequence { events, period }
    }

    // TODO: Replace this with `impl Index`.
    // TODO: Why does this return a slice?
    // TODO: Take a relative instant.
    /// Returns all events that fall on a given instant.
    pub(crate) fn get(&self, timestamp: sample::Instant) -> &[Event] {
        let timestamp = timestamp + self.period.start.since_start;

        if !self.period.contains(timestamp) {
            return &[];
        }

        self.events.get(timestamp)
    }
}
