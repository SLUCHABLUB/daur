use crate::view::Axis;

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
