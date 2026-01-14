//! Types pertaining to selection.

use crate::Id;
use crate::note;
use crate::project::Track;
use crate::project::track::clip;
use mitsein::hash_set1::HashSet1;
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
    /// The underlying stack.
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
    pub fn take_tracks(&mut self) -> Option<HashSet1<Id<Track>>> {
        let set: HashSet<_> = self
            .items
            .drain(..)
            .map(|item| match item {
                Selectable::Track(track) => track,
                Selectable::Clip(clip) => clip.track,
                Selectable::Note(note) => note.clip.track,
            })
            .collect();

        HashSet1::try_from(set).ok()
    }

    /// Takes all clips out of the selection.
    pub fn take_clips(&mut self) -> Option<HashSet1<clip::Path>> {
        let mut set = HashSet::new();

        for item in &mut self.items {
            match *item {
                Selectable::Track(_) => (),
                Selectable::Clip(clip) => {
                    set.insert(clip);
                    *item = Selectable::Track(clip.track);
                }
                Selectable::Note(note) => {
                    set.insert(note.clip);
                    *item = Selectable::Track(note.clip.track);
                }
            }
        }

        HashSet1::try_from(set).ok()
    }

    /// Takes all notes out of the selection.
    pub fn take_notes(&mut self) -> Option<HashSet1<note::Path>> {
        let mut set = HashSet::new();

        for item in &mut self.items {
            match *item {
                Selectable::Track(_) | Selectable::Clip(_) => (),
                Selectable::Note(note) => {
                    set.insert(note);
                    *item = Selectable::Clip(note.clip);
                }
            }
        }

        HashSet1::try_from(set).ok()
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
