//! Types pertaining to [`UserInterface`].

pub mod relative;

mod colour;
mod direction;
mod grid;
mod length;
mod offset;
mod point;
mod rectangle;
mod size;
mod theme;
mod vector;

pub use colour::Colour;
pub use direction::Direction;
pub use grid::Grid;
pub use length::{Length, NonZeroLength};
pub use offset::Offset;
pub use point::Point;
pub use rectangle::Rectangle;
pub use size::Size;
pub use theme::{Theme, ThemeColour};
pub use vector::Vector;

use crate::View;
use crate::view::RenderArea;
use std::path::Path;

/// A user interface for the DAW.
#[cfg_attr(doc, doc(hidden))]
pub trait UserInterface {
    /// The default depth of the black keys on the piano roll.
    const BLACK_KEY_DEPTH: NonZeroLength;
    /// The border thickness of a [bordered view](View::Bordered).
    ///
    /// This should be the same for thick and non-thick borders.
    /// Padding may be added to non-thick borders.
    const BORDER_THICKNESS: Length;
    /// The default width of a [grid](Grid) cell.
    const CELL_WIDTH: NonZeroLength;
    /// The default width of the piano-roll keys.
    const KEY_WIDTH: NonZeroLength;
    /// The default depth of the piano-roll piano.
    const PIANO_DEPTH: NonZeroLength;
    /// The width of the playback button.
    const PLAYBACK_BUTTON_WIDTH: NonZeroLength;
    /// The default height of the project bar.
    const PROJECT_BAR_HEIGHT: NonZeroLength;
    /// The height of the ruler.
    const RULER_HEIGHT: NonZeroLength;
    /// The default width for the track settings.
    const TRACK_SETTINGS_WITH: NonZeroLength;

    /// Exits the DAW.
    ///
    /// It is OK for implementations not to do anything or restart when this is run.
    /// This may be the case if the application, for example, cannot close itself.
    fn exit(&mut self);

    /// Returns the current screen size.
    #[must_use]
    fn size(&self) -> Size;

    // TODO: make this optional?
    /// Returns the position of the mouse.
    #[must_use]
    fn mouse_position(&self) -> Point;

    /// Returns the width of the string
    #[must_use]
    fn string_width(string: &str) -> Length;

    /// Returns the height of the string
    #[must_use]
    fn string_height(string: &str) -> Length;

    /// Returns the width of the title if it was to be applied to the view.
    #[must_use]
    fn title_width(title: &str, titled: &View) -> Length;

    /// Returns the height of the title if it was to be applied to the view.
    #[must_use]
    fn title_height(title: &str, titled: &View) -> Length;

    /// Returns the default size for a file selector.
    #[must_use]
    fn file_selector_size(path: &Path) -> Size;

    /// Returns the [rendering area](RenderArea) of the user interface.
    #[must_use]
    fn render_area(&self) -> RenderArea {
        RenderArea {
            area: Rectangle {
                position: Point::ZERO,
                size: self.size(),
            },
            mouse_position: self.mouse_position(),
        }
    }
}
