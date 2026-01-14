//! Items pertaining to [`Changing`].

use crate::metre::Instant;
use crate::metre::NonZeroInstant;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;

// TODO: Use a `serde(with)` that uses `Into<T> for &Self` instead of the `Clone` bound.
/// A setting that changes over time.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Default, Serialize, Deserialize)]
#[serde(from = "Serial<T>", into = "Serial<T>")]
#[serde(bound(serialize = "T: Serialize + Clone"))]
#[serde(bound(deserialize = "T: Deserialize<'de>"))]
pub struct Changing<T> {
    /// The starting value.
    pub start: T,
    /// The changes.
    #[serde(default)]
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

impl<T> From<T> for Changing<T> {
    fn from(start: T) -> Changing<T> {
        Changing {
            start,
            changes: BTreeMap::new(),
        }
    }
}

/// The serial representation of [`Changing`].
#[derive(Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum Serial<T> {
    /// A single value.
    Constant(T),
    /// A starting value and timeline of changes.
    Changing {
        /// The starting value.
        start: T,
        /// The timeline of changes to the value.
        changes: BTreeMap<NonZeroInstant, T>,
    },
}

impl<T> From<Changing<T>> for Serial<T> {
    fn from(changing: Changing<T>) -> Self {
        let Changing { start, changes } = changing;

        if changes.is_empty() {
            Serial::Constant(start)
        } else {
            Serial::Changing { start, changes }
        }
    }
}

impl<T> From<Serial<T>> for Changing<T> {
    fn from(serial: Serial<T>) -> Self {
        match serial {
            Serial::Constant(start) => Changing {
                start,
                changes: BTreeMap::new(),
            },
            Serial::Changing { start, changes } => Changing { start, changes },
        }
    }
}
