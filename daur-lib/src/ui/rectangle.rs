use crate::ui::{Length, Point, Size, Vector};
use crate::view::{Direction, Quotum};
use std::cmp::{max, min};
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
    /// Returns whether the rectangle contains a point.
    #[must_use]
    pub fn contains(self, point: Point) -> bool {
        let bottom_right = self.bottom_right();

        let xs = self.position.x..bottom_right.x;
        let ys = self.position.y..bottom_right.y;

        xs.contains(&point.x) && ys.contains(&point.y)
    }

    /// Returns the bottom right corner of the rectangle.
    ///
    /// If discrete [lengths](Length) are used, this position is *not* within the rectangle.
    #[must_use]
    pub fn bottom_right(self) -> Point {
        Point {
            x: self.position.x + self.size.width,
            y: self.position.y + self.size.height,
        }
    }

    #[must_use]
    fn from_points(position: Point, bottom_right: Point) -> Option<Rectangle> {
        if bottom_right.x < position.x || bottom_right.y < position.y {
            return None;
        }

        let width = bottom_right.x - position.x;
        let height = bottom_right.y - position.y;

        Some(Rectangle {
            position,
            size: Size { width, height },
        })
    }

    /// Calculates the intersection between two rectangles.
    #[must_use]
    pub fn intersection(self, other: Rectangle) -> Option<Rectangle> {
        let position = Point {
            x: max(self.position.x, other.position.x),
            y: max(self.position.y, other.position.y),
        };

        let bottom_right = Point {
            x: min(self.bottom_right().x, other.bottom_right().x),
            y: min(self.bottom_right().y, other.bottom_right().y),
        };

        Rectangle::from_points(position, bottom_right)
    }

    /// Splits the rectangle.
    pub fn split(
        self,
        direction: Direction,
        quota: &[Quotum],
    ) -> impl Iterator<Item = Rectangle> + use<'_> {
        let orthogonal = self.size.orthogonal_to(direction);

        // the size that will be allocated to `Quotum::Remaining` quota
        let mut fill_size = self.size.parallel_to(direction);

        let mut fill_count: u32 = 0;

        for quotum in quota {
            match quotum {
                Quotum::Remaining => fill_count = fill_count.saturating_add(1),
                Quotum::Exact(length) => fill_size -= *length,
                Quotum::DirectionDependent(size) => fill_size -= size.parallel_to(direction),
            }
        }

        if let Some(fill_count) = NonZeroU32::new(fill_count) {
            fill_size /= fill_count;
        }

        let mut offset = Length::ZERO;

        quota.iter().filter_map(move |quotum| {
            let parallel = match quotum {
                Quotum::Remaining => fill_size,
                Quotum::Exact(length) => *length,
                Quotum::DirectionDependent(size) => size.parallel_to(direction),
            };

            let position = self.position + Vector::directed(offset, direction);

            offset += parallel;

            self.intersection(Rectangle {
                position,
                size: Size::from_parallel_orthogonal(parallel, orthogonal, direction),
            })
        })
    }
}
