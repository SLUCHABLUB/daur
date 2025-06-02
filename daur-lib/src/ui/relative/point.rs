use crate::ui;
use crate::ui::Length;
use std::ops::Add;

/// A point on the screen, relative to some other point.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Point {
    /// The (relative) x coordinate of the point.
    pub x: Length,
    /// The (relative) y coordinate of the point.
    pub y: Length,
}

impl Add<Point> for ui::Point {
    type Output = ui::Point;

    fn add(mut self, rhs: Point) -> ui::Point {
        self.x += rhs.x;
        self.y += rhs.y;
        self
    }
}
