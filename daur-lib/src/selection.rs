use crate::project::Track;
use crate::project::track::Clip;
use crate::{Id, Note};
use getset::{CopyGetters, Setters};

// TODO: selecting multiple tracks and clips
/// The selection state of the app.
#[derive(Clone, Debug, Setters, CopyGetters)]
pub struct Selection {
    /// The selected track.
    pub track: Id<Track>,
    /// The selected clip.
    pub clips: Vec<Id<Clip>>,

    /// The selected notes.
    pub notes: Vec<Id<Note>>,
}

impl Default for Selection {
    fn default() -> Self {
        Selection {
            track: Id::NONE,
            clips: Vec::new(),
            notes: Vec::new(),
        }
    }
}
