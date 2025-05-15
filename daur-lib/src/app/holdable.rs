use crate::metre::Instant;
use crate::ui::{Length, Point};
use crate::{App, UserInterface};

/// An object that can be held.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[must_use = "use `Holdable::let_go`"]
pub enum HoldableObject {
    /// The handle/title bar of the piano roll.
    PianoRollHandle {
        /// How far down, on the handle, it was grabbed.
        y: Length,
    },
    /// A note in the process of being created.
    NoteCreation {
        /// Where the note should start.
        start: Instant,
    },
}

impl HoldableObject {
    /// Moves the object.
    pub(crate) fn update<Ui: UserInterface>(self, app: &mut App<Ui>, position: Point) {
        match self {
            HoldableObject::PianoRollHandle { y } => {
                app.piano_roll.content_height =
                    app.ui.size().height - position.y + y - Length::PIXEL;
            }
            HoldableObject::NoteCreation { .. } => (),
        }
    }
}
