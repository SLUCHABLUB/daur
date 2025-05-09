use core::sync::atomic::{AtomicUsize, Ordering};
use getset::CopyGetters;

/// An identifier for a popup.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, CopyGetters)]
pub struct Id {
    /// The numeric id.
    #[get_copy = "pub(crate)"]
    inner: usize,
}

impl Id {
    /// Generates a new identifier.
    pub(crate) fn generate() -> Id {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);

        Id {
            inner: COUNTER.fetch_add(1, Ordering::Relaxed),
        }
    }
}
