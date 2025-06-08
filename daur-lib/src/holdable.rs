use crate::metre::Instant;
use crate::project::track::clip;
use crate::ui::{Length, Point};

/// An object that can be held.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[remain::sorted]
pub enum Holdable {
    /// A clip in the track workspace.
    Clip(clip::Path),
    /// A note in the process of being created.
    NoteCreation {
        /// Where the note should start.
        start: Instant,
    },
    /// The title bar of the piano roll.
    PianoRollHandle {
        /// How far down, on the handle, it was grabbed.
        y: Length,
    },
    /// A rectangular selection box in the piano roll.
    SelectionBox {
        /// The point where the selection started.
        start: Point,
    },
}
