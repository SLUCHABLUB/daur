use crate::measure::Length;
use ratatui::layout::Spacing;
use saturating_cast::SaturatingCast as _;
use std::ops::{Add, AddAssign, Mul, Neg, Sub};

/// A signed [`Length`](crate::measure::length)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Offset {
    inner: i32,
}

impl Offset {
    const fn new(value: i32) -> Offset {
        Offset { inner: value }
    }

    /// 0
    pub const ZERO: Offset = Offset::new(0);

    /// Returns the absolute value of self
    #[must_use]
    pub fn abs(self) -> Length {
        if self.inner.is_negative() {
            -self
        } else {
            self
        }
        .saturate()
    }

    /// Convert `self` to a [`Length`] by saturating
    #[must_use]
    pub fn saturate(self) -> Length {
        Length::new(self.inner.saturating_cast())
    }

    /// Converts self to a [`Length`] if it fits
    #[must_use]
    pub fn to_length(self) -> Option<Length> {
        Some(Length::new(u16::try_from(self.inner).ok()?))
    }
}

impl From<Length> for Offset {
    fn from(length: Length) -> Self {
        Offset::new(i32::from(length.inner()))
    }
}

impl From<&Spacing> for Offset {
    fn from(spacing: &Spacing) -> Self {
        match spacing {
            Spacing::Space(space) => Offset::new(i32::from(*space)),
            Spacing::Overlap(overlap) => -Offset::new(i32::from(*overlap)),
        }
    }
}

impl Add for Offset {
    type Output = Offset;

    fn add(self, rhs: Self) -> Self::Output {
        Offset {
            inner: self.inner.saturating_add(rhs.inner),
        }
    }
}

impl Sub for Offset {
    type Output = Offset;

    fn sub(self, rhs: Self) -> Self::Output {
        Offset {
            inner: self.inner.saturating_sub(rhs.inner),
        }
    }
}

impl Neg for Offset {
    type Output = Offset;

    fn neg(self) -> Self::Output {
        Offset {
            inner: self.inner.saturating_neg(),
        }
    }
}

impl Add<Length> for Offset {
    type Output = Offset;

    fn add(self, rhs: Length) -> Self::Output {
        self + Offset::from(rhs)
    }
}

impl Sub<Length> for Offset {
    type Output = Offset;

    fn sub(self, rhs: Length) -> Self::Output {
        self - Offset::from(rhs)
    }
}

impl Mul<i32> for Offset {
    type Output = Offset;

    fn mul(self, rhs: i32) -> Self::Output {
        Offset {
            inner: self.inner.saturating_mul(rhs),
        }
    }
}

impl Mul<usize> for Offset {
    type Output = Offset;

    fn mul(self, rhs: usize) -> Self::Output {
        self * rhs.saturating_cast::<i32>()
    }
}

impl AddAssign<Length> for Offset {
    fn add_assign(&mut self, rhs: Length) {
        *self = *self + rhs;
    }
}

impl AddAssign<Offset> for Length {
    fn add_assign(&mut self, rhs: Offset) {
        *self = (rhs + *self).saturate();
    }
}
