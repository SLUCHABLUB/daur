use crate::metre::Instant;
use crate::ui::{Length, Point};

/// An object that can be held.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[must_use = "use `Holdable::let_go`"]
#[remain::sorted]
pub enum HoldableObject {
    /// A note in the process of being created.
    NoteCreation {
        /// Where the note should start.
        start: Instant,
    },
    /// The handle/title bar of the piano roll.
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
