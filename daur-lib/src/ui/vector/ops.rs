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

impl<R: Into<Ratio>> Mul<R> for Vector {
    type Output = Vector;

    fn mul(mut self, rhs: R) -> Vector {
        self *= rhs;
        self
    }
}

impl<N: Into<NonZeroRatio>> Div<N> for Vector {
    type Output = Vector;

    fn div(mut self, rhs: N) -> Vector {
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

impl<R: Into<Ratio>> MulAssign<R> for Vector {
    fn mul_assign(&mut self, rhs: R) {
        let rhs = rhs.into();
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<N: Into<NonZeroRatio>> DivAssign<N> for Vector {
    fn div_assign(&mut self, rhs: N) {
        let rhs = rhs.into();

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
