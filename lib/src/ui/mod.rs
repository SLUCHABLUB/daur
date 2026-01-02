//! Types pertaining to [`UserInterface`].

pub mod relative;

mod colour;
mod direction;
mod length;
mod offset;
mod point;
mod rectangle;
mod settings;
mod size;
mod theme;
mod vector;

pub use colour::Colour;
pub use direction::Direction;
pub use length::Length;
pub use length::NonZeroLength;
pub use offset::Offset;
pub use point::Point;
pub use rectangle::Rectangle;
pub use size::Size;
pub use theme::Theme;
pub use theme::ThemeColour;
pub use vector::Vector;

pub(crate) use settings::Settings;

use crate::view::RenderArea;

/// A user interface for the DAW.
pub trait UserInterface: Sync + 'static {
    /// The default depth of the black keys on the piano roll.
    const BLACK_KEY_DEPTH: NonZeroLength;
    /// The border thickness of a [bordered view](crate::View::Bordered).
    ///
    /// This should be the same for thick and non-thick borders.
    /// Padding may be added to non-thick borders.
    const BORDER_THICKNESS: Length;
    /// The default width of a [quantisation](crate::metre::Quantisation) cell.
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
    /// The amount of padding to apply to all sides of a title in a [title bar](crate::View::TitleBar).
    const TITLE_PADDING: Length;
    /// The default width for the track settings.
    const TRACK_SETTINGS_WITH: NonZeroLength;

    /// Exits the DAW.
    ///
    /// It is OK for implementations to not do anything or to restart when this is run.
    /// This may be needed if the application, for example, cannot close itself.
    fn exit(&self);

    /// Returns the current screen size.
    #[must_use]
    fn size(&self) -> Size;

    // TODO: make this optional?
    /// Returns the position of the mouse.
    #[must_use]
    fn mouse_position(&self) -> Point;

    /// Returns the width of the string.
    #[must_use]
    fn string_width(string: &str) -> Length;

    /// Returns the height of the string.
    #[must_use]
    fn string_height(string: &str) -> Length;

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
