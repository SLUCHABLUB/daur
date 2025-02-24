use crate::ui::Length;
use ratatui::layout::Position;

/// A point on the screen
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct Point {
    /// The x coordinate of the point
    pub x: Length,
    /// The y coordinate of the point
    pub y: Length,
}

impl Point {
    /// The origin point, in the top-left corner of the screen
    pub const ZERO: Point = Point {
        x: Length::ZERO,
        y: Length::ZERO,
    };

    pub(crate) fn from_position(position: Position) -> Self {
        Point {
            x: Length::new(position.x),
            y: Length::new(position.y),
        }
    }

    pub(crate) fn to_position(self) -> Position {
        Position {
            x: self.x.inner(),
            y: self.y.inner(),
        }
    }
}
