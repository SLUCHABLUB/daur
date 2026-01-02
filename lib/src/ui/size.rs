use crate::ui::Length;
use crate::view::Axis;
use crate::view::Quotum2D;

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
    pub fn parallel_to(self, axis: Axis) -> Length {
        match axis {
            Axis::X => self.width,
            Axis::Y => self.height,
        }
    }

    /// Returns the length orthogonal to a direction.
    #[must_use]
    pub fn orthogonal_to(self, axis: Axis) -> Length {
        match axis {
            Axis::X => self.height,
            Axis::Y => self.width,
        }
    }

    /// Construct a size from two lengths,
    /// one parallel and one orthogonal to a direction.
    #[must_use]
    pub fn from_parallel_orthogonal(parallel: Length, orthogonal: Length, axis: Axis) -> Size {
        match axis {
            Axis::X => Size {
                width: parallel,
                height: orthogonal,
            },
            Axis::Y => Size {
                width: orthogonal,
                height: parallel,
            },
        }
    }

    /// Converts the size to a [quotum](Quotum2D).
    #[must_use]
    pub const fn quotum(self) -> Quotum2D {
        Quotum2D {
            x: self.width.quotum(),
            y: self.height.quotum(),
        }
    }
}
