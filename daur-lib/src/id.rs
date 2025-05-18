use alloc::sync::Weak;
use core::fmt;
use core::fmt::{Debug, Formatter};
use core::hash::{Hash, Hasher};
use core::marker::PhantomData;
use core::sync::atomic::{AtomicUsize, Ordering};

/// An identifier.
pub struct Id<Item> {
    /// The numeric id.
    inner: usize,
    phantom: PhantomData<Weak<Item>>,
}

impl<Item> Id<Item> {
    /// Generates a new identifier.
    pub(crate) fn generate() -> Self {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);

        Id {
            inner: COUNTER.fetch_add(1, Ordering::Relaxed),
            phantom: PhantomData,
        }
    }
}

impl<Item> Copy for Id<Item> {}

impl<Item> Clone for Id<Item> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<Item> Eq for Id<Item> {}

impl<Item> PartialEq for Id<Item> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<Item> Hash for Id<Item> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

impl<Item> Debug for Id<Item> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}
