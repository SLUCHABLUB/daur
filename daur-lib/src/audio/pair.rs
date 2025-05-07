use crate::audio::Sample;
use std::ops::{Add, AddAssign};

/// A sample pair.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct Pair {
    /// The left sample.
    pub left: Sample,
    /// The right sample.
    pub right: Sample,
}

impl Pair {
    /// 0
    pub const ZERO: Pair = Pair {
        left: Sample::ZERO,
        right: Sample::ZERO,
    };
}

impl From<Sample> for Pair {
    fn from(sample: Sample) -> Self {
        Pair {
            left: sample,
            right: sample,
        }
    }
}

impl From<(f64, f64)> for Pair {
    fn from((left, right): (f64, f64)) -> Self {
        Pair {
            left: Sample::new(left),
            right: Sample::new(right),
        }
    }
}

impl Add for Pair {
    type Output = Pair;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign for Pair {
    fn add_assign(&mut self, rhs: Self) {
        self.left += rhs.left;
        self.right += rhs.right;
    }
}
