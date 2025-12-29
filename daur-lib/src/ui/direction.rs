use crate::ui::Length;
use crate::ui::Offset;
use crate::ui::Vector;
use crate::view::Axis;
use std::ops::Mul;
use std::ops::Neg;

/// A direction in which items can be laid out
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Direction {
    /// From the bottom to the top
    Up,
    /// From the right to the left
    Left,
    /// From the top to the bottom
    Down,
    /// From the left to the right
    Right,
}

impl Direction {
    /// Returns the axis along which the direction lies.
    #[must_use]
    pub fn axis(self) -> Axis {
        match self {
            Direction::Left | Direction::Right => Axis::X,
            Direction::Down | Direction::Up => Axis::Y,
        }
    }

    /// Returns whether the direction is "negative" (up or left).
    #[must_use]
    pub fn is_negative(self) -> bool {
        match self {
            Direction::Up | Direction::Left => true,
            Direction::Down | Direction::Right => false,
        }
    }
}

impl Neg for Direction {
    type Output = Direction;

    fn neg(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
        }
    }
}

impl Mul<Length> for Direction {
    type Output = Vector;

    fn mul(self, rhs: Length) -> Vector {
        let mut x = Offset::ZERO;
        let mut y = Offset::ZERO;

        match self {
            Direction::Up => y = Offset::negative(rhs),
            Direction::Left => x = Offset::negative(rhs),
            Direction::Down => y = Offset::positive(rhs),
            Direction::Right => x = Offset::positive(rhs),
        }

        Vector { x, y }
    }
}
