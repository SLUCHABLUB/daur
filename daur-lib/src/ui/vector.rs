use crate::ui::{Length, Offset};
use crate::widget::Direction;
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
