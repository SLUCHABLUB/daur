//! Items pertaining to project history.

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

// TODO: Add a method for constructing an undoing action
/// A performed [edit](Edit).
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
        /// The position of the clip.
        start: Instant,
        /// The deleted clip.
        clip: Clip,
    },
    /// The deletion of a note.
    DeleteNote {
        /// The position of the note.
        instant: relative::Instant,
        /// The pitch of the note.
        pitch: Pitch,
        /// The deleted note.
        note: Note,
    },
    /// The deletion of a track.
    DeleteTrack {
        /// The index of the track.
        index: usize,
        /// The deleted track.
        track: Track,
    },
    /// The insertion of a clip.
    InsertClip(clip::Path),
    /// The insertion of a note.
    InsertNote(note::Path),
    /// The relocation of a clip.
    MoveClip {
        /// The track from which the clip was moved.
        original_track: Id<Track>,
        /// The position from which the clip was moved.
        original_position: Instant,
        /// The path to the clip after the move.
        new_path: clip::Path,
    },
    /// The setting of the key.
    SetKey {
        /// The position at which the key was set.
        at: Instant,
        /// The key that was set.
        to: Key,
        /// The key that was overwritten.
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
