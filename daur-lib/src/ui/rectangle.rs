use crate::ui::point::Point;
use crate::ui::Size;
use ratatui::layout::{Constraint, Direction, Flex, Layout, Rect, Spacing};

/// A rectangle on the screen
#[derive(Copy, Clone, Default, Debug)]
pub struct Rectangle {
    /// The top left point of the rectangle
    pub position: Point,
    /// The size of the rectangle
    pub size: Size,
}

impl Rectangle {
    /// Whether the rectangle contains `point`
    #[must_use]
    pub fn contains(self, point: Point) -> bool {
        self.to_rect().contains(point.to_position())
    }

    /// The intersection between `self` and `other`
    #[must_use]
    pub fn intersection(self, other: Rectangle) -> Rectangle {
        Rectangle::from_rect(self.to_rect().intersection(other.to_rect()))
    }

    pub(crate) fn from_rect(rect: Rect) -> Self {
        Rectangle {
            position: Point::from_position(rect.as_position()),
            size: Size::from_size(rect.as_size()),
        }
    }

    pub(crate) fn to_rect(self) -> Rect {
        Rect::from((self.position.to_position(), self.size.to_size()))
    }

    /// Split the rect based on the layout specification
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
