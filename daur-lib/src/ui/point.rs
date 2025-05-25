use crate::ui::{Length, Vector, relative};
use std::ops::{Add, AddAssign, Sub, SubAssign};

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

// TODO: derive
impl Add<Vector> for Point {
    type Output = Point;

    fn add(mut self, rhs: Vector) -> Self::Output {
        self += rhs;
        self
    }
}

// TODO: derive
impl AddAssign<Vector> for Point {
    fn add_assign(&mut self, rhs: Vector) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

// TODO: derive
impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(mut self, rhs: Vector) -> Self::Output {
        self -= rhs;
        self
    }
}

// TODO: derive
impl SubAssign<Vector> for Point {
    fn sub_assign(&mut self, rhs: Vector) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
