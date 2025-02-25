use crate::ui::Length;
use crate::widget::Direction;
use ratatui::layout;

/// The size of something on the screen
#[derive(Copy, Clone, Debug, Default)]
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

    pub(crate) fn from_size(size: layout::Size) -> Size {
        Size {
            width: Length::new(size.width),
            height: Length::new(size.height),
        }
    }

    pub(crate) fn to_size(self) -> layout::Size {
        layout::Size {
            width: self.width.inner(),
            height: self.height.inner(),
        }
    }

    /// Returns the length parallel to `direction`
    #[must_use]
    pub fn parallel_to(self, direction: Direction) -> Length {
        match direction {
            Direction::Left | Direction::Right => self.width,
            Direction::Up | Direction::Down => self.height,
        }
    }

    /// Returns the length orthogonal to `direction`
    #[must_use]
    pub fn orthogonal_to(self, direction: Direction) -> Length {
        match direction {
            Direction::Left | Direction::Right => self.height,
            Direction::Up | Direction::Down => self.width,
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
}
