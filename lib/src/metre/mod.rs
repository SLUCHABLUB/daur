//! Types pertaining to [musical time](https://en.wikipedia.org/wiki/Metre_(music)).
//!
//! It is important to note that they represent note values and not beats.
//! The duration of a beat depends on the time signature.
//! In common time it is a quarter note whilst in cut time it is a half note.

pub mod relative;

mod changing;
mod duration;
mod instant;
mod measure;
mod offset_mapping;
mod period;
mod quantisation;
mod time_context;
mod time_signature;

pub use changing::Changing;
pub use duration::Duration;
pub use duration::NonZeroDuration;
pub use instant::Instant;
pub use instant::NonZeroInstant;
pub use measure::Measure;
pub use offset_mapping::OffsetMapping;
pub use period::NonZeroPeriod;
pub use period::Period;
pub use quantisation::Quantisation;
pub use time_context::TimeContext;
pub use time_signature::TimeSignature;
