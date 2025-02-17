use crate::length::Length;
use ratatui::layout::Position;

#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub struct Point {
    pub x: Length,
    pub y: Length,
}

impl Point {
    pub fn from_position(position: Position) -> Self {
        Point {
            x: Length::new(position.x),
            y: Length::new(position.y),
        }
    }

    pub fn to_position(self) -> Position {
        Position {
            x: self.x.inner.0,
            y: self.y.inner.0,
        }
    }
}
