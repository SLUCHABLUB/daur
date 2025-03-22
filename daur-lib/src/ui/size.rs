use crate::ui::{Length, Point, Vector};
use crate::view::Direction;

/// The size of something on the screen
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct Size {
    /// The width of the thing
    pub width: Length,
    /// The height of the thing
    pub height: Length,
}

impl Size {
    /// 0 by 0
    pub const ZERO: Size = Size {
        width: Length::ZERO,
        height: Length::ZERO,
    };

    /// Returns the length parallel to a direction.
    #[must_use]
    pub fn parallel_to(self, direction: Direction) -> Length {
        match direction {
            Direction::Left | Direction::Right => self.width,
            Direction::Up | Direction::Down => self.height,
        }
    }

    /// Returns the length orthogonal to a direction.
    #[must_use]
    pub fn orthogonal_to(self, direction: Direction) -> Length {
        match direction {
            Direction::Left | Direction::Right => self.height,
            Direction::Up | Direction::Down => self.width,
        }
    }

    /// Construct a size from two lengths,
    /// one parallel and one orthogonal to a direction.
    #[must_use]
    pub fn from_parallel_orthogonal(
        parallel: Length,
        orthogonal: Length,
        direction: Direction,
    ) -> Size {
        match direction {
            Direction::Left | Direction::Right => Size {
                width: parallel,
                height: orthogonal,
            },
            Direction::Up | Direction::Down => Size {
                width: orthogonal,
                height: parallel,
            },
        }
    }

    /// Returns the vector from the top-left corner to the bottom-right corner
    /// of a rectangle with this size.
    #[must_use]
    pub fn diagonal(self) -> Vector {
        Point {
            x: self.width,
            y: self.height,
        }
        .position()
    }
}
