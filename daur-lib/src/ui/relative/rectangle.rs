use crate::ui::{Length, Size, relative};
use std::cmp::{max, min};

/// A rectangle on the screen relative to some point.
#[derive(Copy, Clone, Default, Debug)]
pub struct Rectangle {
    /// The top left point of the rectangle.
    pub position: relative::Point,
    /// The size of the rectangle.
    pub size: Size,
}

impl Rectangle {
    #[must_use]
    pub(crate) fn containing_both(first: relative::Point, second: relative::Point) -> Rectangle {
        let position = relative::Point {
            x: min(first.x, second.x),
            y: min(first.y, second.y),
        };

        let bottom_right = relative::Point {
            x: max(first.x, second.x) + Length::PIXEL,
            y: max(first.y, second.y) + Length::PIXEL,
        };

        let width = bottom_right.x - position.x;
        let height = bottom_right.y - position.y;

        Rectangle {
            position,
            size: Size { width, height },
        }
    }
}
