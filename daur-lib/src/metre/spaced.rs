use crate::metre::Instant;
use alloc::collections::BTreeMap;

/// A list of items that are spaced out in (musical) time.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Spaced<T> {
    entries: BTreeMap<Instant, T>,
}

impl<T> Spaced<T> {
    /// Constructs a new spaced list.
    #[must_use]
    pub const fn new() -> Self {
        Spaced {
            entries: BTreeMap::new(),
        }
    }

    /// Tries inserting an element into the list at the given position.
    ///
    /// # Errors
    ///
    /// If there is already an element at the position,
    /// the item which was attempted to be inserted is returned.
    pub fn try_insert(&mut self, at: Instant, item: T) -> Result<(), T> {
        if self.entries.contains_key(&at) {
            return Err(item);
        }

        self.entries.insert(at, item);

        Ok(())
    }

    /// Returns an iterator over the list.
    #[must_use]
    pub fn iter(&self) -> impl ExactSizeIterator<Item = (Instant, &T)> {
        self.entries.iter().map(|(instant, item)| (*instant, item))
    }

    /// Returns an iterator over mutable references to the list.
    #[must_use]
    pub fn iter_mut(&mut self) -> impl ExactSizeIterator<Item = (Instant, &mut T)> {
        self.entries
            .iter_mut()
            .map(|(instant, item)| (*instant, item))
    }
}

impl<T> Default for Spaced<T> {
    fn default() -> Self {
        Spaced::new()
    }
}
