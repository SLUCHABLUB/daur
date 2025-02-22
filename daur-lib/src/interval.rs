/// An interval between two pitches.
///
/// Note: No `Eq` or `PartialEq` implementation is provided.
/// This is due to "equality"/"equivalence" is not semantically ubiquitous.
/// To compare intervals, the `semitones` and `kind` methods may be used.
#[derive(Copy, Clone, Debug)]
pub struct Interval {
    semitones: i16,
}

impl Interval {
    pub const PERFECT_UNISON: Interval = Interval::from_semitones(0);

    pub const fn from_semitones(semitones: i16) -> Interval {
        Interval { semitones }
    }

    pub fn semitones(self) -> i16 {
        self.semitones
    }
}
