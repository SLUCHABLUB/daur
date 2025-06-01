use crate::metre::Instant;
use crate::note;
use crate::note::Key;
use crate::project::track::{Clip, clip};
use crate::project::{Track, track};
use mitsein::iter1::{FromIterator1, IntoIterator1};
use mitsein::vec1::Vec1;

// TODO: method for constructing an undoing action
#[expect(dead_code, reason = "see TODO")]
#[derive(Debug)]
#[remain::sorted]
pub enum HistoryEntry {
    /// The addition of a track at the bottom.
    AddTrack(track::Id),
    /// A collection of actions that were taken at once.
    Cluster(Vec1<HistoryEntry>),
    /// The deletion of some clips.
    DeleteClip {
        start: Instant,
        clip: Clip,
    },
    /// The deletion of a track.
    DeleteTrack {
        index: usize,
        track: Track,
    },
    /// The insertion of a clip.
    InsertClip(clip::Id),
    InsertNote(note::Id),
    SetKey {
        at: Instant,
        to: Key,
        from: Option<Key>,
    },
}

impl FromIterator1<HistoryEntry> for HistoryEntry {
    fn from_iter1<I>(items: I) -> Self
    where
        I: IntoIterator1<Item = HistoryEntry>,
    {
        HistoryEntry::Cluster(items.into_iter1().collect1())
    }
}
