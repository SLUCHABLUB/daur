use crate::metre::NonZeroDuration;

// TODO: pitch-bends?
/// A [note](https://en.wikipedia.org/wiki/Musical_note).
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Note {
    /// The duration of the note.
    pub duration: NonZeroDuration,
    // TODO: articulation
}
