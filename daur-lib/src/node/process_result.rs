use crate::Audio;

#[must_use]
pub struct ProcessResult<'samples> {
    /// The output audio.
    pub audio: Audio<'samples>,
    /// Whether the processor has more audio to output, even if no new input is received.
    pub should_continue: bool,
}
