use crate::ui::Offset;
use std::ops::Neg;

/// A vector
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
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

    /// Reflects the vector along the x = y line
    #[must_use]
    pub const fn reflection(self) -> Vector {
        Vector {
            x: self.y,
            y: self.x,
        }
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Vector {
            x: -self.x,
            y: -self.y,
        }
    }
}
