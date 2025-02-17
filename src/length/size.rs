use crate::length::Length;
use ratatui::layout::Direction;

#[derive(Copy, Clone)]
pub struct Size {
    pub width: Length,
    pub height: Length,
}

impl Size {
    pub const ZERO: Size = Size {
        width: Length::ZERO,
        height: Length::ZERO,
    };

    pub fn parallel_to(self, direction: Direction) -> Length {
        match direction {
            Direction::Horizontal => self.width,
            Direction::Vertical => self.height,
        }
    }
}
