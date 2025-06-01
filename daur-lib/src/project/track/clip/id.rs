use crate::project::track;
use getset::CopyGetters;
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::atomic::{AtomicU32, Ordering};

/// An identifier for a clip during runtime.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, CopyGetters)]
pub struct Id {
    /// The id of the containing track.
    #[get_copy = "pub"]
    track: track::Id,
    /// The numeric id.
    inner: u32,
}

impl Id {
    /// Generates a new identifier.
    pub(crate) fn generate(track: track::Id) -> Self {
        static COUNTER: AtomicU32 = AtomicU32::new(0);

        Id {
            track,
            inner: COUNTER.fetch_add(1, Ordering::Relaxed),
        }
    }
}
