//! Typer pertaining to [`Holdable`]

mod window_size;

pub use window_size::WindowSide;

use crate::metre::Instant;
use crate::project::track::clip;
use crate::ui::{Length, Point};
use crate::{Id, Popup};

/// An object that can be held.
#[cfg_attr(doc, doc(hidden))]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[remain::sorted]
pub enum Holdable {
    // TODO: add a relative point
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
    /// A side of a popup to resize it.
    ResizePopup {
        /// The popup being resized.
        popup: Id<Popup>,
        /// The side that is grabbed.
        side: WindowSide,
    },
    /// A rectangular selection box in the piano roll.
    SelectionBox {
        /// The point where the selection started.
        start: Point,
    },
}
