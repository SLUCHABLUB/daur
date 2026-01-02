use crate::ui::Length;
use crate::ui::Point;
use crate::ui::Size;
use crate::ui::Vector;
use std::cmp::max;
use std::cmp::min;
use std::ops::Add;
use std::ops::AddAssign;

/// A rectangle on the screen.
#[derive(Copy, Clone, Default, Debug)]
pub struct Rectangle {
    /// The top left point of the rectangle.
    pub position: Point,
    /// The size of the rectangle.
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
    pub(crate) fn containing_both(first: Point, second: Point) -> Rectangle {
        let position = Point {
            x: min(first.x, second.x),
            y: min(first.y, second.y),
        };

        let bottom_right = Point {
            x: max(first.x, second.x) + Length::PIXEL,
            y: max(first.y, second.y) + Length::PIXEL,
        };

        let width = bottom_right.x - position.x;
        let height = bottom_right.y - position.y;

        Rectangle {
            position,
            size: Size { width, height },
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

    /// Returns whether two rectangles intersect.
    #[must_use]
    pub fn intersects(self, other: Rectangle) -> bool {
        self.intersection(other).is_some()
    }
}

impl Add<Vector> for Rectangle {
    type Output = Rectangle;

    fn add(mut self, rhs: Vector) -> Rectangle {
        self += rhs;
        self
    }
}

impl AddAssign<Vector> for Rectangle {
    fn add_assign(&mut self, rhs: Vector) {
        self.position += rhs;
    }
}
