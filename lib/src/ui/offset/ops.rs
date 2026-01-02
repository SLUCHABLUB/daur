use crate::NonZeroRatio;
use crate::Ratio;
use crate::ui::Offset;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Neg;
use std::ops::Sub;
use std::ops::SubAssign;

impl<O: Into<Offset>> Add<O> for Offset {
    type Output = Offset;

    fn add(self, rhs: O) -> Offset {
        Offset {
            pixels: self.pixels.saturating_add(rhs.into().pixels),
        }
    }
}

impl<O: Into<Offset>> Sub<O> for Offset {
    type Output = Offset;

    fn sub(self, rhs: O) -> Offset {
        Offset {
            pixels: self.pixels.saturating_sub(rhs.into().pixels),
        }
    }
}

impl<R: Into<Ratio>> Mul<R> for Offset {
    type Output = Offset;

    fn mul(self, rhs: R) -> Offset {
        let length = self.abs() * rhs;

        if self.pixels.is_negative() {
            Offset::negative(length)
        } else {
            Offset::positive(length)
        }
    }
}

impl<N: Into<NonZeroRatio>> Div<N> for Offset {
    type Output = Offset;

    fn div(self, rhs: N) -> Offset {
        #![expect(clippy::suspicious_arithmetic_impl, reason = "we take the reciprocal")]
        self * rhs.into().reciprocal().get()
    }
}

// --- ASSIGNMENT OPERATIONS ---

impl<O: Into<Offset>> AddAssign<O> for Offset {
    fn add_assign(&mut self, rhs: O) {
        *self = *self + rhs;
    }
}

impl<O: Into<Offset>> SubAssign<O> for Offset {
    fn sub_assign(&mut self, rhs: O) {
        *self = *self - rhs;
    }
}

impl<R: Into<Ratio>> MulAssign<R> for Offset {
    fn mul_assign(&mut self, rhs: R) {
        *self = *self * rhs;
    }
}

impl<N: Into<NonZeroRatio>> DivAssign<N> for Offset {
    fn div_assign(&mut self, rhs: N) {
        *self = *self / rhs;
    }
}

// --- UNARY OPERATIONS ---

impl Neg for Offset {
    type Output = Offset;

    fn neg(self) -> Offset {
        Offset {
            pixels: self.pixels.saturating_neg(),
        }
    }
}
