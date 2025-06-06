use crate::project::Track;
use crate::project::track::clip;
use crate::{Id, note};
use std::collections::HashSet;

/// An item that can be selected.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Selectable {
    /// A track.
    Track(Id<Track>),
    /// A clip.
    Clip(clip::Path),
    /// A note.
    Note(note::Path),
}

/// A selection stack.
#[derive(Clone, Debug, Default)]
pub struct Selection {
    items: Vec<Selectable>,
}

impl Selection {
    /// Clears the selection.
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Returns whether the selection contains a track.
    pub fn contains_track(&self, track: Id<Track>) -> bool {
        self.items.iter().any(|item| match *item {
            Selectable::Track(id) => id == track,
            Selectable::Clip(id) => id.track == track,
            Selectable::Note(note) => note.clip.track == track,
        })
    }

    /// Returns whether the selection contains a clip.
    pub fn contains_clip(&self, clip: clip::Path) -> bool {
        self.items.iter().any(|item| match *item {
            Selectable::Track(_) => false,
            Selectable::Clip(id) => id == clip,
            Selectable::Note(note) => note.clip == clip,
        })
    }

    /// Adds an item to the top of the selection stack.
    pub fn push(&mut self, item: Selectable) {
        self.items.push(item);
    }

    /// Adds a track to the top of the selection stack.
    pub fn push_track(&mut self, track: Id<Track>) {
        self.items.push(Selectable::Track(track));
    }

    /// Adds a clip to the top of the selection stack.
    pub fn push_clip(&mut self, clip: clip::Path) {
        self.items.push(Selectable::Clip(clip));
    }

    /// Takes all tracks out of the selection.
    // TODO: return Option<HashSet1>
    pub fn take_tracks(&mut self) -> Option<HashSet<Id<Track>>> {
        let set: HashSet<_> = self
            .items
            .drain(..)
            .map(|item| match item {
                Selectable::Track(track) => track,
                Selectable::Clip(clip) => clip.track,
                Selectable::Note(note) => note.clip.track,
            })
            .collect();

        (!set.is_empty()).then_some(set)
    }

    // TODO: replace clips with tracks
    /// Takes all clips out of the selection.
    // TODO: return Option<HashSet1>
    pub fn take_clips(&mut self) -> Option<HashSet<clip::Path>> {
        let mut set = HashSet::new();

        self.items.retain(|item| match *item {
            Selectable::Track(_) => true,
            Selectable::Clip(clip) => {
                set.insert(clip);
                false
            }
            Selectable::Note(note) => {
                set.insert(note.clip);
                false
            }
        });

        (!set.is_empty()).then_some(set)
    }

    // TODO: replace notes with clips
    /// Takes all notes out of the selection.
    // TODO: return Option<HashSet1>
    pub fn take_notes(&mut self) -> Option<HashSet<note::Path>> {
        let mut set = HashSet::new();

        self.items.retain(|item| match *item {
            Selectable::Track(_) | Selectable::Clip(_) => true,
            Selectable::Note(note) => {
                set.insert(note);
                false
            }
        });

        (!set.is_empty()).then_some(set)
    }

    /// Returns the track selected last.
    pub fn top_track(&self) -> Option<Id<Track>> {
        self.items.last().map(|item| match *item {
            Selectable::Track(track) => track,
            Selectable::Clip(clip) => clip.track,
            Selectable::Note(note) => note.clip.track,
        })
    }

    /// Returns the clip selected last.
    pub fn top_clip(&self) -> Option<clip::Path> {
        self.items.iter().rev().find_map(|item| match *item {
            Selectable::Track(_) => None,
            Selectable::Clip(clip) => Some(clip),
            Selectable::Note(note) => Some(note.clip),
        })
    }
}
