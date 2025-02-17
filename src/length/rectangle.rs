use crate::length::point::Point;
use crate::length::Length;
use ratatui::layout::{Constraint, Direction, Flex, Layout, Rect, Spacing};

#[derive(Copy, Clone, Default)]
pub struct Rectangle {
    pub x: Length,
    pub y: Length,
    pub width: Length,
    pub height: Length,
}

impl Rectangle {
    pub fn contains(self, point: Point) -> bool {
        self.to_rect().contains(point.to_position())
    }

    pub fn intersection(self, other: Rectangle) -> Rectangle {
        Rectangle::from_rect(self.to_rect().intersection(other.to_rect()))
    }

    pub fn from_rect(rect: Rect) -> Self {
        Rectangle {
            x: Length::new(rect.x),
            y: Length::new(rect.y),
            width: Length::new(rect.width),
            height: Length::new(rect.height),
        }
    }

    pub fn to_rect(self) -> Rect {
        Rect {
            x: self.x.inner.0,
            y: self.y.inner.0,
            width: self.width.inner.0,
            height: self.height.inner.0,
        }
    }

    // TODO: move away from ratatui types
    pub fn split<Constraints: IntoIterator<Item = Constraint>>(
        self,
        constraints: Constraints,
        direction: Direction,
        flex: Flex,
        spacing: &Spacing,
    ) -> impl Iterator<Item = Rectangle> {
        #[expect(clippy::unnecessary_to_owned, reason = "false positive")]
        Layout::new(direction, constraints)
            .flex(flex)
            .spacing(spacing.clone())
            .split(self.to_rect())
            .to_vec()
            .into_iter()
            .map(Rectangle::from_rect)
    }
}
