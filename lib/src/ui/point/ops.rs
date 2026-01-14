//! Implementations of mathematical operations on [`Point`].

use crate::ui::Point;
use crate::ui::Vector;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;

// --- INFIX OPERATIONS ---

impl Add<Vector> for Point {
    type Output = Point;

    fn add(mut self, rhs: Vector) -> Point {
        self += rhs;
        self
    }
}

impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(mut self, rhs: Vector) -> Point {
        self -= rhs;
        self
    }
}

// --- ASSIGNMENT OPERATIONS ---

impl AddAssign<Vector> for Point {
    fn add_assign(&mut self, rhs: Vector) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign<Vector> for Point {
    fn sub_assign(&mut self, rhs: Vector) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
