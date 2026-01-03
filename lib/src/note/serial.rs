use crate::Id;
use crate::Note;
use crate::metre::NonZeroDuration;
use crate::metre::relative;
use crate::note::Pitch;
use serde::Deserialize;
use serde::Serialize;

#[derive(Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub(crate) struct Serial {
    pub position: relative::Instant,
    pub pitch: Pitch,
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
