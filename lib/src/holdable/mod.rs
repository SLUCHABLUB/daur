//! Items pertaining to [`Holdable`].

mod window_size;

pub use window_size::WindowSide;

use crate::Id;
use crate::Popup;
use crate::metre::Instant;
use crate::project::track::clip;
use crate::ui::Length;
use crate::ui::Point;
use crate::ui::relative;

/// An object that can be held.
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
    /// A popup, to move it.
    Popup {
        /// The id of the popup.
        id: Id<Popup>,
        /// The point, relative to the popup, where it was grabbed.
        point: relative::Point,
    },
    /// A side of a popup, to resize it.
    PopupSide {
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
