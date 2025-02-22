use crate::measure::Length;
use ratatui::layout::Direction;

/// The size of something on the screen
#[derive(Copy, Clone, Debug)]
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

    /// Returns the length parallel to `direction`
    #[must_use]
    pub fn parallel_to(self, direction: Direction) -> Length {
        match direction {
            Direction::Horizontal => self.width,
            Direction::Vertical => self.height,
        }
    }

    /// Returns the length orthogonal to `direction`
    #[must_use]
    pub fn orthogonal_to(self, direction: Direction) -> Length {
        match direction {
            Direction::Horizontal => self.height,
            Direction::Vertical => self.width,
        }
    }

    /// Construct a `Size` from two distances,
    /// one parallel and one orthogonal to `direction`.
    #[must_use]
    pub fn from_parallel_orthogonal(
        parallel: Length,
        orthogonal: Length,
        direction: Direction,
    ) -> Size {
        match direction {
            Direction::Horizontal => Size {
                width: parallel,
                height: orthogonal,
            },
            Direction::Vertical => Size {
                width: orthogonal,
                height: parallel,
            },
        }
    }
}
