use crate::Id;
use crate::project::Track;
use crate::project::track::Clip;
use getset::CopyGetters;
use std::fmt::Debug;
use std::hash::Hash;

/// A path to a clip.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, CopyGetters)]
pub struct Path {
    /// The id of the containing track.
    pub track: Id<Track>,
    /// The id of the clip.
    pub clip: Id<Clip>,
}

impl Path {
    /// Constructs a new path.
    #[must_use]
    pub fn new(track: Id<Track>, clip: Id<Clip>) -> Path {
        Path { track, clip }
    }
}
