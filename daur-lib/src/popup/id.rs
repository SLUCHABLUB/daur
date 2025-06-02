use std::fmt::Debug;
use std::hash::Hash;
use std::sync::atomic::{AtomicU32, Ordering};

/// An identifier for a popup.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Id {
    /// The numeric id.
    inner: u32,
}

impl Id {
    /// Generates a new identifier.
    pub(crate) fn generate() -> Id {
        static COUNTER: AtomicU32 = AtomicU32::new(0);

        Id {
            inner: COUNTER.fetch_add(1, Ordering::Relaxed),
        }
    }
}
