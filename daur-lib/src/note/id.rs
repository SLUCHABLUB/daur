use crate::project::track::clip;
use getset::CopyGetters;
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::atomic::{AtomicU32, Ordering};

/// An identifier for a clip during runtime.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, CopyGetters)]
pub struct Id {
    /// The id of the containing clip.
    #[get_copy = "pub"]
    clip: clip::Id,
    /// The numeric id.
    inner: u32,
}

impl Id {
    /// Generates a new identifier.
    pub(crate) fn generate(clip: clip::Id) -> Self {
        static COUNTER: AtomicU32 = AtomicU32::new(0);

        Id {
            clip,
            inner: COUNTER.fetch_add(1, Ordering::Relaxed),
        }
    }

    pub(crate) fn to_u32(self) -> u32 {
        self.inner
    }
}
