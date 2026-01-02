use crate::Id;
use crate::Note;
use crate::metre::Instant;
use crate::metre::relative;
use crate::note;
use crate::note::Key;
use crate::note::Pitch;
use crate::project::Track;
use crate::project::track::Clip;
use crate::project::track::clip;
use mitsein::iter1::FromIterator1;
use mitsein::iter1::IntoIterator1;
use mitsein::vec1::Vec1;

// TODO: method for constructing an undoing action
#[expect(dead_code, reason = "see TODO")]
#[derive(Debug)]
#[remain::sorted]
pub enum HistoryEntry {
    /// The addition of a track at the bottom.
    AddTrack(Id<Track>),
    /// A collection of actions that were taken at once.
    Cluster(Vec1<HistoryEntry>),
    /// The deletion of a clip.
    DeleteClip {
        start: Instant,
        clip: Clip,
    },
    /// The deletion of a note.
    DeleteNote {
        instant: relative::Instant,
        pitch: Pitch,
        note: Note,
    },
    /// The deletion of a track.
    DeleteTrack {
        index: usize,
        track: Track,
    },
    /// The insertion of a clip.
    InsertClip(clip::Path),
    InsertNote(note::Path),
    MoveClip {
        original_track: Id<Track>,
        original_position: Instant,
        new_path: clip::Path,
    },
    SetKey {
        at: Instant,
        to: Key,
        from: Option<Key>,
    },
}

impl FromIterator1<HistoryEntry> for HistoryEntry {
    fn from_iter1<I>(items: I) -> HistoryEntry
    where
        I: IntoIterator1<Item = HistoryEntry>,
    {
        HistoryEntry::Cluster(items.into_iter1().collect1())
    }
}
