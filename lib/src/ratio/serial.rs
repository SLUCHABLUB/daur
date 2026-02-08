//! Items pertaining to [`Serial`].

use crate::NonZeroRatio;
use crate::Ratio;
use anyhow::bail;
use serde::Deserialize;
use serde::Serialize;
use std::num::NonZeroU64;

/// The serial representation of a [ratio](Ratio).
#[derive(Copy, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub(super) enum Serial {
    /// A floating point number.
    Float(f64),
    /// An integer.
    I64(i64),
    /// A natural number.
    U64(u64),

    // These are here since serde by default does not try to derserialise a `u64` or `i64`
    // from a `u128` or `i128` even if it is in range (as of writing).
    /// A big (according to serde) integer.
    I128(i128),
    /// A big (according to serde) natural number.
    U128(u128),

    /// A map with "numerator" and "denominator" keys.
    Map {
        /// The numerator.
        numerator: u64,
        /// The denominator.
        denominator: NonZeroU64,
    },
}

/// Utility macro for returning with an error.
macro_rules! invalid_value {
    ($value:expr) => {
        bail!("invalid value: {}, expected {EXPECTED}", $value)
    };
}

impl From<Ratio> for Serial {
    fn from(value: Ratio) -> Self {
        let Ratio {
            numerator,
            denominator,
        } = value;

        // TODO: Use `Serial::Float` if we can.
        Serial::Map {
            numerator,
            denominator,
        }
    }
}

impl TryFrom<Serial> for Ratio {
    type Error = anyhow::Error;

    fn try_from(serial: Serial) -> Result<Self, Self::Error> {
        const EXPECTED: &str = "a non-negative rational number";

        match serial {
            Serial::Float(value) => {
                if value.is_nan() || value < 0.0 || value.is_infinite() {
                    invalid_value!(value);
                }

                Ok(Ratio::approximate(value))
            }
            Serial::I64(value) => match u64::try_from(value) {
                Ok(value) => Ratio::try_from(Serial::U64(value)),
                Err(_negative) => invalid_value!(value),
            },
            Serial::U64(value) => Ok(Ratio::integer(value)),
            Serial::I128(value) => match u128::try_from(value) {
                Ok(value) => Ratio::try_from(Serial::U128(value)),
                Err(_negative) => invalid_value!(value),
            },
            Serial::U128(value) => match u64::try_from(value) {
                Ok(value) => Ratio::try_from(Serial::U64(value)),
                Err(_overflow) => bail!("value too big: {value}"),
            },
            Serial::Map {
                numerator,
                denominator,
            } => Ok(Ratio {
                numerator,
                denominator,
            }),
        }
    }
}

impl TryFrom<Serial> for NonZeroRatio {
    type Error = anyhow::Error;

    fn try_from(serial: Serial) -> Result<Self, Self::Error> {
        const EXPECTED: &str = "a positive rational number";

        match serial {
            Serial::Float(value) => {
                if value.is_nan() || value <= 0.0 || value.is_infinite() {
                    invalid_value!(value);
                }

                let Some(ratio) = NonZeroRatio::from_ratio(Ratio::approximate(value)) else {
                    invalid_value!(value);
                };

                Ok(ratio)
            }
            Serial::I64(value) => match u64::try_from(value) {
                Ok(value) => NonZeroRatio::try_from(Serial::U64(value)),
                Err(_negative) => invalid_value!(value),
            },
            Serial::U64(value) => match NonZeroU64::new(value) {
                Some(value) => Ok(NonZeroRatio::integer(value)),
                None => invalid_value!(value),
            },
            Serial::I128(value) => match u128::try_from(value) {
                Ok(value) => NonZeroRatio::try_from(Serial::U128(value)),
                Err(_negative) => invalid_value!(value),
            },
            Serial::U128(value) => match u64::try_from(value) {
                Ok(value) => NonZeroRatio::try_from(Serial::U64(value)),
                Err(_overflow) => bail!("value too big: {value}"),
            },
            Serial::Map {
                numerator,
                denominator,
            } => match NonZeroU64::new(numerator) {
                Some(numerator) => Ok(NonZeroRatio::new(numerator, denominator)),
                None => invalid_value!(format!("{numerator} / {denominator}")),
            },
        }
    }
}
