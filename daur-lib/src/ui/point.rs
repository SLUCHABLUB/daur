use crate::ui::{Length, Offset, Vector};
use ratatui::layout::Position;
use std::ops::{Add, AddAssign, Sub};

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

impl Add<Vector> for Point {
    type Output = Point;

    fn add(mut self, rhs: Vector) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign<Vector> for Point {
    fn add_assign(&mut self, rhs: Vector) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            x: Offset::from(self.x) - Offset::from(rhs.x),
            y: Offset::from(self.y) - Offset::from(rhs.y),
        }
    }
}
