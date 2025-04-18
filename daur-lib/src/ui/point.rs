use crate::ui::{Length, Offset, Vector};
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

    /// Returns the [position](wikipedia.org/wiki/Position_(geometry)) of the point.
    #[must_use]
    pub const fn position(self) -> Vector {
        Vector {
            x: Offset::positive(self.x),
            y: Offset::positive(self.y),
        }
    }
}

// TODO: derive
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
        self.position() - rhs.position()
    }
}
