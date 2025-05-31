use crate::ui::Vector;
use crate::{NonZeroRatio, Ratio};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

// --- INFIX OPERATIONS ---

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(mut self, rhs: Vector) -> Vector {
        self += rhs;
        self
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(mut self, rhs: Vector) -> Vector {
        self -= rhs;
        self
    }
}

impl Mul<Ratio> for Vector {
    type Output = Vector;

    fn mul(mut self, rhs: Ratio) -> Vector {
        self *= rhs;
        self
    }
}

impl Div<NonZeroRatio> for Vector {
    type Output = Vector;

    fn div(mut self, rhs: NonZeroRatio) -> Vector {
        self /= rhs;
        self
    }
}

// --- ASSIGNMENT OPERATIONS ---

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign for Vector {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl MulAssign<Ratio> for Vector {
    fn mul_assign(&mut self, rhs: Ratio) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl DivAssign<NonZeroRatio> for Vector {
    fn div_assign(&mut self, rhs: NonZeroRatio) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

// --- UNARY OPERATIONS ---

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector {
            x: -self.x,
            y: -self.y,
        }
    }
}
