use crate::ui::{Point, Rectangle, Size};
use std::cmp::{max, min};

/// A side of a window that can be grabbed to resize it.
/// This does not include the top part as it is used for moving the window.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum WindowSide {
    /// The top-left corner.
    TopLeft,
    /// The top-right corner.
    TopRight,
    /// The left side.
    Left,
    /// The right side.
    Right,
    /// The bottom-left corner.
    BottomLeft,
    /// The bottom side.
    Bottom,
    /// The bottom-right corner.
    BottomRight,
}

impl WindowSide {
    pub(crate) fn resize(self, rectangle: Rectangle, point: Point) -> Rectangle {
        let original_top_left = rectangle.position;
        let original_bottom_right = rectangle.bottom_right();

        let mut top_left = original_top_left;
        let mut bottom_right = original_bottom_right;

        match self {
            WindowSide::TopLeft => top_left = point,
            WindowSide::TopRight => {
                bottom_right.x = point.x;
                top_left.y = point.y;
            }
            WindowSide::Left => top_left.x = point.x,
            WindowSide::Right => bottom_right.x = point.x,
            WindowSide::BottomLeft => {
                top_left.x = point.x;
                bottom_right.y = point.y;
            }
            WindowSide::Bottom => bottom_right.y = point.y,
            WindowSide::BottomRight => bottom_right = point,
        }

        top_left.x = min(top_left.x, original_bottom_right.x);
        top_left.y = min(top_left.y, original_bottom_right.y);

        bottom_right.x = max(bottom_right.x, original_top_left.x);
        bottom_right.y = max(bottom_right.y, original_top_left.y);

        let size = Size {
            width: bottom_right.x - top_left.x,
            height: bottom_right.y - top_left.y,
        };

        Rectangle {
            position: top_left,
            size,
        }
    }
}
