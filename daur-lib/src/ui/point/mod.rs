mod ops;

use crate::ui::Length;
use crate::ui::relative;

/// A point on the screen
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
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

    #[must_use]
    pub(crate) fn relative_to(self, other: Point) -> relative::Point {
        relative::Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
