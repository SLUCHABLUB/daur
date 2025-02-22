use crate::measure::point::Point;
use crate::measure::Length;
use ratatui::layout::{Constraint, Direction, Flex, Layout, Rect, Spacing};

/// A rectangle on the screen
#[derive(Copy, Clone, Default, Debug)]
pub struct Rectangle {
    /// The distance from the left of the screen to the rectangle
    pub x: Length,
    /// The distance from the top of the screen to the rectangle
    pub y: Length,
    /// The width of the rectangle
    pub width: Length,
    /// The height of the rectangle
    pub height: Length,
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
            x: Length::new(rect.x),
            y: Length::new(rect.y),
            width: Length::new(rect.width),
            height: Length::new(rect.height),
        }
    }

    pub(crate) fn to_rect(self) -> Rect {
        Rect {
            x: self.x.inner(),
            y: self.y.inner(),
            width: self.width.inner(),
            height: self.height.inner(),
        }
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
