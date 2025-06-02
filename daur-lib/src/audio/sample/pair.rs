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
    fn from(sample: Sample) -> Pair {
        Pair {
            left: sample,
            right: sample,
        }
    }
}

impl From<[Sample; 2]> for Pair {
    fn from([left, right]: [Sample; 2]) -> Pair {
        Pair { left, right }
    }
}

impl Add for Pair {
    type Output = Pair;

    fn add(mut self, rhs: Pair) -> Pair {
        self += rhs;
        self
    }
}

impl AddAssign for Pair {
    fn add_assign(&mut self, rhs: Pair) {
        self.left += rhs.left;
        self.right += rhs.right;
    }
}
