pub mod offset;
pub mod point;
pub mod rectangle;
pub mod size;

use crate::time::Ratio;
use ratatui::layout::Constraint;
use saturating_cast::SaturatingCast as _;
use std::num::Saturating;
use std::ops::{Add, AddAssign, Div, Mul, Sub};

/// An abstract orthogonal distance between two points
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Length {
    inner: Saturating<u16>,
}

impl Length {
    pub const ZERO: Length = Length {
        inner: Saturating(0),
    };

    pub const DOUBLE_BORDER: Length = Length {
        inner: Saturating(2),
    };

    pub const CURSOR_WIDTH: Length = Length::CHAR_WIDTH;

    pub const CELL: Length = Length {
        inner: Saturating(4),
    };

    pub const CHAR_WIDTH: Length = Length {
        inner: Saturating(1),
    };

    pub const CHAR_HEIGHT: Length = Length::CHAR_WIDTH;

    pub fn string_height(string: &str) -> Self {
        // TODO: use graphemes
        let length = string.lines().count();
        let length = length.saturating_cast();
        Length {
            inner: Saturating(length),
        }
    }

    pub fn string_width(string: &str) -> Self {
        // TODO: use graphemes
        let length = string.chars().count();
        let length = length.saturating_cast();
        Length {
            inner: Saturating(length),
        }
    }

    pub fn constraint(self) -> Constraint {
        Constraint::Length(self.inner.0)
    }

    pub fn constraint_max(self) -> Constraint {
        Constraint::Max(self.inner.0)
    }
}

impl Add for Length {
    type Output = Length;

    fn add(self, rhs: Length) -> Self::Output {
        Length {
            inner: self.inner + rhs.inner,
        }
    }
}

impl AddAssign for Length {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Length {
    type Output = Length;

    fn sub(self, rhs: Length) -> Self::Output {
        Length {
            inner: self.inner - rhs.inner,
        }
    }
}

impl Mul<Ratio> for Length {
    type Output = Length;

    fn mul(self, rhs: Ratio) -> Self::Output {
        let length = (Ratio::new(self.inner.0.into(), 1) * rhs).round();
        let length = length.saturating_cast();
        Length {
            inner: Saturating(length),
        }
    }
}

impl Mul<u32> for Length {
    type Output = Length;

    fn mul(self, rhs: u32) -> Self::Output {
        self * Ratio::new(rhs, 1)
    }
}

impl Mul<usize> for Length {
    type Output = Length;

    fn mul(self, rhs: usize) -> Self::Output {
        self * rhs.saturating_cast::<u32>()
    }
}

// TODO: require rhs to be NonZero
impl Div for Length {
    type Output = Ratio;

    fn div(self, rhs: Length) -> Self::Output {
        Ratio::new(u32::from(self.inner.0), u32::from(rhs.inner.0))
    }
}

impl Div<u32> for Length {
    type Output = Length;

    fn div(self, rhs: u32) -> Self::Output {
        #![expect(
            clippy::suspicious_arithmetic_impl,
            reason = "we multiply by the reciprocal"
        )]
        self * Ratio::new(1, rhs)
    }
}
