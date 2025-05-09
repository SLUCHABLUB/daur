use crate::musical_time::{Instant, Spaced};
use crate::pitch::Pitch;
use mitsein::index_map1::IndexMap1;

/// Values spaced in both time and pitch.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct PitchSpaced<T> {
    inner: Spaced<IndexMap1<Pitch, T>>,
}

impl<T> PitchSpaced<T> {
    /// Constructs a new pitch space.
    #[must_use]
    pub const fn new() -> PitchSpaced<T> {
        PitchSpaced {
            inner: Spaced::new(),
        }
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = (Instant, Pitch, &T)> {
        self.inner.iter().flat_map(|(instant, pitches)| {
            pitches
                .as_index_map()
                .iter()
                .map(move |(pitch, item)| (instant, *pitch, item))
        })
    }
}

impl<T> Default for PitchSpaced<T> {
    fn default() -> Self {
        PitchSpaced::new()
    }
}
