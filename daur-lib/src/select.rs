use crate::note;
use crate::project::track;
use crate::project::track::clip;
use std::collections::HashSet;

/// An item that can be selected.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Selectable {
    /// A track.
    Track(track::Id),
    /// A clip.
    Clip(clip::Id),
    /// A note.
    Note(note::Id),
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
    pub fn contains_track(&self, track: track::Id) -> bool {
        self.items.iter().any(|item| match *item {
            Selectable::Track(id) => id == track,
            Selectable::Clip(id) => id.track() == track,
            Selectable::Note(note) => note.clip().track() == track,
        })
    }

    /// Returns whether the selection contains a clip.
    pub fn contains_clip(&self, clip: clip::Id) -> bool {
        self.items.iter().any(|item| match *item {
            Selectable::Track(_) => false,
            Selectable::Clip(id) => id == clip,
            Selectable::Note(note) => note.clip() == clip,
        })
    }

    /// Adds an item to the top of the selection stack.
    pub fn push(&mut self, item: Selectable) {
        self.items.push(item);
    }

    /// Adds a track to the top of the selection stack.
    pub fn push_track(&mut self, track: track::Id) {
        self.items.push(Selectable::Track(track));
    }

    /// Takes all tracks out of the selection.
    // TODO: return Option<HashSet1>
    pub fn take_tracks(&mut self) -> Option<HashSet<track::Id>> {
        let set: HashSet<_> = self
            .items
            .drain(..)
            .map(|item| match item {
                Selectable::Track(track) => track,
                Selectable::Clip(clip) => clip.track(),
                Selectable::Note(note) => note.clip().track(),
            })
            .collect();

        (!set.is_empty()).then_some(set)
    }

    /// Takes all clips out of the selection.
    // TODO: return Option<HashSet1>
    pub fn take_clips(&mut self) -> Option<HashSet<clip::Id>> {
        let mut set = HashSet::new();

        self.items.retain(|item| match *item {
            Selectable::Track(_) => true,
            Selectable::Clip(clip) => {
                set.insert(clip);
                false
            }
            Selectable::Note(note) => {
                set.insert(note.clip());
                false
            }
        });

        (!set.is_empty()).then_some(set)
    }

    /// Takes all notes out of the selection.
    // TODO: return Option<HashSet1>
    pub fn take_notes(&mut self) -> Option<HashSet<note::Id>> {
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
    pub fn top_track(&self) -> Option<track::Id> {
        self.items.last().map(|item| match *item {
            Selectable::Track(track) => track,
            Selectable::Clip(clip) => clip.track(),
            Selectable::Note(note) => note.clip().track(),
        })
    }

    /// Returns the clip selected last.
    pub fn top_clip(&self) -> Option<clip::Id> {
        self.items.iter().rev().find_map(|item| match *item {
            Selectable::Track(_) => None,
            Selectable::Clip(clip) => Some(clip),
            Selectable::Note(note) => Some(note.clip()),
        })
    }
}
