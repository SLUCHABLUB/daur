use crate::metre::Instant;
use crate::note;
use crate::note::Key;
use crate::project::track::{Clip, clip};
use crate::project::{Track, track};
use mitsein::btree_map1::BTreeMap1;

// TODO: method for constructing an undoing action
#[expect(dead_code, reason = "see TODO")]
#[derive(Debug)]
#[remain::sorted]
pub enum HistoryEntry {
    /// The addition of a track at the bottom.
    AddTrack(track::Id),
    /// The deletion of some clips.
    DeleteClips(BTreeMap1<Instant, Clip>),
    /// The deletion of a track.
    DeleteTracks(BTreeMap1<usize, Track>),
    /// The insertion of a clip.
    InsertClip(clip::Id),
    InsertNote(note::Id),
    SetKey {
        at: Instant,
        to: Key,
        from: Option<Key>,
    },
}
