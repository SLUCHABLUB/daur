//! Items pertaining to [`Serial`].

use crate::Id;
use crate::Note;
use crate::metre::NonZeroDuration;
use crate::metre::relative;
use crate::note::Pitch;
use serde::Deserialize;
use serde::Serialize;

/// The serial representation of a [`Note`].
#[derive(Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub(crate) struct Serial {
    /// The position.
    pub position: relative::Instant,
    /// The pitch.
    pub pitch: Pitch,
    /// The duration.
    pub duration: NonZeroDuration,
}

impl From<Serial> for Note {
    fn from(serial: Serial) -> Self {
        Note {
            id: Id::generate(),
            duration: serial.duration,
        }
    }
}
