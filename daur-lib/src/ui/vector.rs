use crate::Ratio;
use crate::ui::{Offset, Point};
use core::ops::Mul;
use derive_more::{Add, AddAssign, Neg, Sub, SubAssign};

/// A vector
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Add, Sub, Neg, AddAssign, SubAssign)]
pub struct Vector {
    /// The x coordinate of the vector
    pub x: Offset,
    /// The x coordinate of the vector
    pub y: Offset,
}

impl Vector {
    /// (0, 0)
    pub const ZERO: Vector = Vector {
        x: Offset::ZERO,
        y: Offset::ZERO,
    };

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
