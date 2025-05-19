use crate::Id;
use crate::project::Track;
use crate::project::track::Clip;
use getset::{CopyGetters, Setters};

/// The selection state of the app.
#[derive(Copy, Clone, Debug, Setters, CopyGetters)]
pub struct Selection {
    #[set = "pub(super)"]
    #[get_copy = "pub(crate)"]
    track: Id<Track>,
    #[set = "pub(super)"]
    #[get_copy = "pub(crate)"]
    clip: Id<Clip>,
    // TODO: add a set of notes, and remove clone
}

impl Default for Selection {
    fn default() -> Self {
        Selection {
            track: Id::NONE,
            clip: Id::NONE,
        }
    }
}
