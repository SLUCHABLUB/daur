use crate::NonZeroRatio;
use crate::Ratio;
use anyhow::bail;
use serde::Deserialize;
use std::num::NonZeroU64;

#[derive(Copy, Clone, Deserialize)]
#[serde(untagged)]
enum Serial {
    Float(f64),
    I64(i64),
    U64(u64),
    I128(i128),
    U128(u128),
    Map {
        numerator: u64,
        denominator: NonZeroU64,
    },
}

macro_rules! invalid_value {
    ($value:expr) => {
        bail!("invalid value: {}, expected {EXPECTED}", $value)
    };
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
