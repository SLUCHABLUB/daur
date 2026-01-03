//! Items pertaining to [`Sample`].

mod duration;
mod instant;
mod period;
mod rate;

pub use duration::Duration;
pub use instant::Instant;
pub use period::Period;
pub use rate::Rate;
pub use rate::ZeroRateError;

use bytemuck::NoUninit;
use serde::Deserialize;
use serde::Serialize;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::ops::Add;
use std::ops::AddAssign;
use thiserror::Error;

/// A 32-bit float sample.
#[derive(Copy, Clone, PartialEq, Default, NoUninit, Serialize, Deserialize)]
#[repr(transparent)]
#[serde(try_from = "f32", into = "f32")]
pub struct Sample {
    // INVARIANT: this is on the interval [-1, 1].
    value: f32,
}

impl Sample {
    /// 0
    pub const ZERO: Sample = Sample { value: 0.0 };

    /// Constructs a new sample from a float.
    ///
    /// If it is not in range, it is clamped.
    #[must_use]
    pub const fn new(value: f32) -> Sample {
        if value.is_nan() {
            Sample::ZERO
        } else {
            Sample {
                value: value.clamp(-1.0, 1.0),
            }
        }
    }

    /// Calculates the average of two samples.
    #[must_use]
    pub const fn average(self, other: Sample) -> Sample {
        Sample {
            value: f32::midpoint(self.value, other.value),
        }
    }

    /// Converts the float sample to an `f32`.
    #[must_use]
    pub const fn to_f32(self) -> f32 {
        self.value
    }
}

// This is fine since the internal float is not NaN.
impl Eq for Sample {}

impl Debug for Sample {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.value.fmt(f)
    }
}

impl From<Sample> for f32 {
    fn from(value: Sample) -> Self {
        value.to_f32()
    }
}

/// An error which can be returned when parsing a sample from a float.
#[derive(Copy, Clone, PartialEq, Debug, Error)]
#[error("the value {0} is not on the interval [-1, 1]")]
pub struct ParseSampleError(pub f32);

impl TryFrom<f32> for Sample {
    type Error = ParseSampleError;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        (-1.0..=1.0)
            .contains(&value)
            .then_some(Sample { value })
            .ok_or(ParseSampleError(value))
    }
}

impl Add for Sample {
    type Output = Sample;

    fn add(self, rhs: Sample) -> Sample {
        Sample::new(self.value + rhs.value)
    }
}

impl AddAssign for Sample {
    fn add_assign(&mut self, rhs: Sample) {
        *self = *self + rhs;
    }
}
