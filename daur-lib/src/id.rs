use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::hash::Hash;
use std::hash::Hasher;
use std::marker::PhantomData;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;

/// An identifier.
pub struct Id<T> {
    number: u32,
    _type: PhantomData<T>,
}

impl<T> Id<T> {
    /// Generates a new identifier.
    pub(crate) fn generate() -> Id<T> {
        static COUNTER: AtomicU32 = AtomicU32::new(0);

        Id {
            number: COUNTER.fetch_add(1, Ordering::Relaxed),
            _type: PhantomData,
        }
    }
}

impl<T> Copy for Id<T> {}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Eq for Id<T> {}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
    }
}

impl<T> Hash for Id<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.number.hash(state);
    }
}

impl<T> Debug for Id<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.number.fmt(f)
    }
}
