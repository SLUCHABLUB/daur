use crate::ui::{Length, Offset, Point, Size, Vector};
use crate::view::{Axis, Quotated};
use crate::{Ratio, UserInterface};
use core::cmp::{max, min};
use core::num::NonZeroU64;
use core::ops::{Add, AddAssign};

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
    /// If discrete [lengths](super::Length) are used, this position is *not* within the rectangle.
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
    #[must_use]
    pub(crate) fn split<Ui: UserInterface>(
        self,
        axis: Axis,
        views: &[Quotated],
    ) -> impl DoubleEndedIterator<Item = Rectangle> + use<'_, Ui> {
        // cache the sizes or None if Quotum::Remaining is used
        let sizes: Vec<Option<Length>> = views
            .iter()
            .map(|quotated| quotated.size_parallel_to::<Ui>(axis))
            .collect();

        let orthogonal = self.size.orthogonal_to(axis);

        // the size that will be allocated to the `Quotum::Remaining` quota
        let mut fill_size = self.size.parallel_to(axis);

        let mut fill_count: u64 = 0;

        for size in &sizes {
            if let Some(size) = *size {
                fill_size -= size;
            } else {
                fill_count = fill_count.saturating_add(1);
            }
        }

        if let Some(fill_count) = NonZeroU64::new(fill_count) {
            fill_size *= Ratio::reciprocal_of(fill_count);
        }

        let mut offset = Offset::ZERO;

        sizes.into_iter().filter_map(move |size| {
            let parallel = size.unwrap_or(fill_size);

            let position = self.position + axis * offset;

            offset += parallel;

            self.intersection(Rectangle {
                position,
                size: Size::from_parallel_orthogonal(parallel, orthogonal, axis),
            })
        })
    }
}

impl Add<Vector> for Rectangle {
    type Output = Rectangle;

    fn add(mut self, rhs: Vector) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign<Vector> for Rectangle {
    fn add_assign(&mut self, rhs: Vector) {
        self.position += rhs;
    }
}
