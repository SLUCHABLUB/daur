//! Types pertaining to [musical time](https://en.wikipedia.org/wiki/Metre_(music)).
//!
//! It is important to note that they represent note values and not beats.
//! The duration of a beat depends on the time signature.
//! In common time it is a quarter note whilst in cut time it is a half note.

pub mod relative;

mod bar;
mod changing;
mod duration;
mod instant;
mod period;
mod time_signature;

pub use bar::Bar;
pub use changing::Changing;
pub use duration::{Duration, NonZeroDuration};
pub use instant::{Instant, NonZeroInstant};
pub use period::{NonZeroPeriod, Period};
pub use time_signature::TimeSignature;
