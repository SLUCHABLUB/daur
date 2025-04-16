//! Types pertaining to [`UserInterface`].

pub use grid::Grid;
pub use length::{Length, NonZeroLength};
pub use mapping::Mapping;
pub use offset::Offset;
pub use point::Point;
pub use rectangle::Rectangle;
pub use size::Size;
pub use vector::Vector;

mod grid;
mod length;
mod mapping;
mod offset;
mod point;
mod rectangle;
mod size;
mod vector;

use crate::View;
use crate::popup::Id;
use arcstr::ArcStr;

/// A user interface for the DAW.
#[doc(hidden)]
pub trait UserInterface {
    /// The default depth of the black keys on the piano roll.
    const BLACK_KEY_DEPTH: NonZeroLength;
    /// The border thickness of a [bordered view](View::Bordered) (times 2).
    const DOUBLE_BORDER_THICKNESS: Length;
    /// The default width of a [grid](Grid) cell.
    const CELL_WIDTH: NonZeroLength;
    /// The default width of the piano-roll keys.
    const KEY_WIDTH: NonZeroLength;
    /// The default depth of the piano-roll piano.
    const PIANO_DEPTH: NonZeroLength;
    /// The default width of the playback button.
    const PLAYBACK_BUTTON_WIDTH: NonZeroLength;
    /// The default height of the project bar.
    const PROJECT_BAR_HEIGHT: NonZeroLength;
    const RULER_HEIGHT: NonZeroLength;
    /// The default width for the track settings.
    const TRACK_SETTINGS_WITH: NonZeroLength;

    /// Exits the DAW.
    ///
    /// It is OK for implementations not to do anything or restart when this is run.
    /// This may be the case if the application, for example, can't close itself.
    fn exit(&self);

    /// Returns the height of the string
    #[must_use]
    fn string_height(string: &str) -> Length;

    /// Returns the width of the string
    #[must_use]
    fn string_width(string: &str) -> Length;

    /// A [RAII](wikipedia.org/wiki/RAII) handle to a popup.
    type PopupHandle;

    /// Opens a popup.
    #[must_use]
    fn open_popup(&self, title: ArcStr, view: View, id: Id) -> Self::PopupHandle;

    /// Closes a popup.
    fn close_popup(&self, handle: Self::PopupHandle);
}
