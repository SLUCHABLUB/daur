use crate::musical_time::Duration;
use crate::pitch::Pitch;

// TODO: pitch-bends?
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Note {
    pub pitch: Pitch,
    pub duration: Duration,
    // TODO: articulation
}
