use crate::metre::Instant;
use crate::note::Key;
use crate::project::Track;
use crate::project::track::Clip;
use crate::{Id, Note};
use mitsein::vec1::Vec1;

// TODO: method for constructing an undoing action
#[expect(dead_code, reason = "see TODO")]
#[derive(Debug)]
#[remain::sorted]
pub enum HistoryEntry {
    /// The addition of a track at the bottom.
    AddTrack(Id<Track>),
    /// The deletion of some clips.
    DeleteClips(Vec1<(Id<Track>, Instant, Clip)>),
    /// The deletion of a track.
    DeleteTrack { index: usize, track: Track },
    /// The insertion of a clip.
    InsertClip { track: Id<Track>, clip: Id<Clip> },
    InsertNote {
        track: Id<Track>,
        clip: Id<Clip>,
        note: Id<Note>,
    },
    SetKey {
        at: Instant,
        to: Key,
        from: Option<Key>,
    },
}
