use crate::metre::Instant;
use crate::{Clip, Track};
use alloc::sync::{Arc, Weak};
use getset::{CloneGetters, Setters};

/// The selection state of the app.
#[derive(Clone, Debug, Default, Setters, CloneGetters)]
pub struct Selection {
    #[set = "pub(super)"]
    #[get_clone = "pub(crate)"]
    track: Weak<Track>,
    #[set = "pub(super)"]
    #[get_clone = "pub(crate)"]
    clip: Weak<Clip>,
}

impl Selection {
    pub(crate) fn resolve_clip_and_position(&self) -> Option<(Instant, Arc<Clip>)> {
        self.track
            .upgrade()?
            .clips
            .iter()
            .find(|(_, clip)| Arc::as_ptr(clip) == self.clip.as_ptr())
            .map(|(position, clip)| (position, Arc::clone(clip)))
    }
}
