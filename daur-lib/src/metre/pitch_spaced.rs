use crate::metre::{Instant, Spaced};
use crate::notes::Pitch;
use mitsein::index_map1::IndexMap1;

/// Values spaced in both time and pitch.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct PitchSpaced<T> {
    columns: Spaced<IndexMap1<Pitch, T>>,
}

impl<T> PitchSpaced<T> {
    /// Constructs a new pitch space.
    #[must_use]
    pub const fn new() -> PitchSpaced<T> {
        PitchSpaced {
            columns: Spaced::new(),
        }
    }

    /// Tris inserting an item at a given instant and pitch.
    ///
    /// # Errors
    ///
    /// If there is already an item at the position,
    /// the item which was attempted to be inserted is returned.
    pub fn try_insert(&mut self, instant: Instant, pitch: Pitch, item: T) -> Result<(), T> {
        match self.columns.get_mut(instant) {
            None => {
                // Since `get` returned `None`, `try_insert` will return `Ok`.
                let _ok = self
                    .columns
                    .try_insert(instant, IndexMap1::from_one((pitch, item)));
                Ok(())
            }
            Some(column) => {
                if column.contains_key(&pitch) {
                    Err(item)
                } else {
                    column.insert(pitch, item);
                    Ok(())
                }
            }
        }
    }

    pub(crate) fn with_pitch(&self, pitch: Pitch) -> impl Iterator<Item = (Instant, &T)> {
        self.columns
            .iter()
            .filter_map(move |(instant, column)| Some((instant, column.get(&pitch)?)))
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = (Instant, Pitch, &T)> {
        self.columns.iter().flat_map(|(instant, pitches)| {
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
