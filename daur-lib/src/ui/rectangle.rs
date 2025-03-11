use crate::ui::{Length, Point, Size, Vector};
use crate::widget::Direction;
use crate::Ratio;
use ratatui::layout;
use ratatui::layout::{Constraint, Flex, Layout, Rect, Spacing};
use saturating_cast::SaturatingCast as _;
use std::iter::from_fn;
use std::num::NonZeroU32;

/// A rectangle on the screen
#[derive(Copy, Clone, Default, Debug)]
pub struct Rectangle {
    /// The top left point of the rectangle
    pub position: Point,
    /// The size of the rectangle
    pub size: Size,
}

impl Rectangle {
    /// Returns whether the rectangle contains `point`.
    #[must_use]
    pub fn contains(self, point: Point) -> bool {
        self.to_rect().contains(point.to_position())
    }

    /// Returns the intersection between `self` and `other`.
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

    /// Splits the rect based on a layout specification.
    // TODO: move away from ratatui types
    pub fn split<Constraints: AsMut<[Constraint]>>(
        self,
        mut constraints: Constraints,
        direction: Direction,
        flex: Flex,
        spacing: &Spacing,
    ) -> impl Iterator<Item = Rectangle> {
        let (direction, reverse) = match direction {
            Direction::Up => (layout::Direction::Vertical, true),
            Direction::Left => (layout::Direction::Horizontal, true),
            Direction::Down => (layout::Direction::Vertical, false),
            Direction::Right => (layout::Direction::Horizontal, false),
        };

        let mut rects = Layout::new(direction, constraints.as_mut().iter())
            .flex(flex)
            .spacing(spacing.clone())
            .split(self.to_rect())
            .to_vec();

        if reverse {
            rects.reverse();
        }

        rects.into_iter().map(Rectangle::from_rect)
    }

    /// Splits `self` into `piece_count` equal pieces.
    pub fn split_equally(
        mut self,
        piece_count: usize,
        direction: Direction,
    ) -> impl Iterator<Item = Rectangle> {
        #![expect(
            clippy::arithmetic_side_effects,
            reason = "decrements are checked to be non-zero"
        )]

        let mut piece_count: u32 = piece_count.saturating_cast();
        let orthogonal_length = self.size.orthogonal_to(direction);

        from_fn(move || {
            let remaining_size = self.size.parallel_to(direction);
            let piece_length = remaining_size * Ratio::reciprocal_of(NonZeroU32::new(piece_count)?);

            // To ensure that the iterator has an exact size, we may return zero-sized rectangles
            if piece_count == 1 || remaining_size == Length::ZERO {
                piece_count -= 1;
                return Some(self);
            }

            let piece_size =
                Size::from_parallel_orthogonal(piece_length, orthogonal_length, direction);
            let piece = Rectangle {
                position: self.position,
                size: piece_size,
            };

            piece_count -= 1;
            self.position += Vector::directed(piece_length, direction);
            self.size = Size::from_parallel_orthogonal(
                remaining_size - piece_length,
                orthogonal_length,
                direction,
            );

            Some(piece)
        })
    }
}
