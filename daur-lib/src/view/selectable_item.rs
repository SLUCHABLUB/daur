use crate::project::Track;
use crate::project::track::Clip;
use crate::{Action, Id, Note};

/// An item that can be selected.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum SelectableItem {
    /// A track.
    Track(Id<Track>),
    /// A clip.
    Clip {
        /// The track in which the clip resides.
        track: Id<Track>,
        /// The clip.
        clip: Id<Clip>,
    },
    /// A note.
    Note {
        /// The track in which the clip resides.
        track: Id<Track>,
        /// The clip in which the note resides.
        clip: Id<Clip>,
        /// The note.
        note: Id<Note>,
    },
}

impl SelectableItem {
    /// Returns an action that selects the item.
    pub fn select(self) -> Action {
        match self {
            SelectableItem::Track(track) => Action::SelectTrack(track),
            SelectableItem::Clip { track, clip } => Action::SelectClip { track, clip },
            SelectableItem::Note { track, clip, note } => Action::SelectNote { track, clip, note },
        }
    }
}
