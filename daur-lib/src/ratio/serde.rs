use crate::Ratio;
use arcstr::format;
use serde::Deserialize;
use serde::de::Error;
use std::fmt::Display;
use std::num::NonZeroU64;

impl<'de> Deserialize<'de> for Ratio {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const EXPECTED: &str = "a non-negative rational number";

        fn invalid_value<E: Error, Value: Display>(value: Value) -> E {
            Error::custom(format!("invalid value: {value}, expected {EXPECTED}"))
        }

        #[derive(Copy, Clone, Deserialize)]
        #[serde(untagged)]
        enum Representation {
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

        fn parse<E: Error>(representation: Representation) -> Result<Ratio, E> {
            match representation {
                Representation::Float(value) => {
                    if value.is_nan() || value < 0.0 || value.is_infinite() {
                        Err(invalid_value(value))
                    } else {
                        Ok(Ratio::approximate(value))
                    }
                }
                Representation::I64(value) => match u64::try_from(value) {
                    Ok(value) => parse(Representation::U64(value)),
                    Err(_negative) => Err(invalid_value(value)),
                },
                Representation::U64(value) => Ok(Ratio::integer(value)),
                Representation::I128(value) => match u128::try_from(value) {
                    Ok(value) => parse(Representation::U128(value)),
                    Err(_negative) => Err(invalid_value(value)),
                },
                Representation::U128(value) => match u64::try_from(value) {
                    Ok(value) => parse(Representation::U64(value)),
                    Err(_overflow) => Err(Error::custom(format!("value too big: {value}"))),
                },
                Representation::Map {
                    denominator,
                    numerator,
                } => Ok(Ratio {
                    numerator,
                    denominator,
                }),
            }
        }

        parse(Representation::deserialize(deserializer)?)
    }
}
