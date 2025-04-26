use crate::Ratio;
use crate::ui::{Length, Offset, Point};
use crate::view::Direction;
use derive_more::{Add, AddAssign, Neg, Sub, SubAssign};
use std::ops::Mul;

/// A vector
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Add, Sub, Neg, AddAssign, SubAssign)]
pub struct Vector {
    /// The x coordinate of the vector
    pub x: Offset,
    /// The x coordinate of the vector
    pub y: Offset,
}

impl Vector {
    /// Construct a new vector with y = 0
    #[must_use]
    pub const fn from_x(x: Offset) -> Vector {
        Vector { x, y: Offset::ZERO }
    }

    /// Construct a new vector with x = 0
    #[must_use]
    pub const fn from_y(y: Offset) -> Vector {
        Vector { x: Offset::ZERO, y }
    }

    // TODO: replace with Mul
    /// Constructs a new vector with a given length and direction
    #[must_use]
    pub const fn directed(length: Length, direction: Direction) -> Vector {
        let mut x = Offset::ZERO;
        let mut y = Offset::ZERO;

        match direction {
            Direction::Up => y = Offset::negative(length),
            Direction::Left => x = Offset::negative(length),
            Direction::Down => y = Offset::positive(length),
            Direction::Right => x = Offset::positive(length),
        }

        Vector { x, y }
    }

    /// Returns the (saturated) endpoint of the vector when placed at the origin.
    #[must_use]
    pub fn point(self) -> Point {
        Point {
            x: self.x.rectify(),
            y: self.y.rectify(),
        }
    }
}

// TODO: derive
impl Mul<Ratio> for Vector {
    type Output = Vector;

    fn mul(self, rhs: Ratio) -> Self::Output {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
