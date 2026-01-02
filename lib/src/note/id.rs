use crate::Id;
use crate::Note;
use crate::project::Track;
use crate::project::track::Clip;
use crate::project::track::clip;
use getset::CopyGetters;
use std::fmt::Debug;
use std::hash::Hash;

/// An identifier for a clip during runtime.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, CopyGetters)]
pub struct Path {
    /// The path to the containing clip.
    pub clip: clip::Path,
    /// The id of the note.
    pub note: Id<Note>,
}

impl Path {
    /// Constructs a new path.
    #[must_use]
    pub fn new(track: Id<Track>, clip: Id<Clip>, note: Id<Note>) -> Path {
        Path {
            clip: clip::Path::new(track, clip),
            note,
        }
    }
}
