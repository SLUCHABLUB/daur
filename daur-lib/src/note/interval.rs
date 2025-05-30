/// An interval between two pitches.
///
/// Note: No [`Eq`] or [`PartialEq`] implementation is provided.
/// This is due to "equality"/"equivalence" is not semantically ubiquitous.
/// To compare intervals, the [`semitones`](Interval::semitones) and [`kind`](todo) methods may be used.
#[derive(Copy, Clone, Debug)]
pub struct Interval {
    semitones: i8,
}

impl Interval {
    /// An interval of 0 semitones.
    pub const PERFECT_UNISON: Interval = Interval::from_semitones(0);

    /// An interval of 1 semitone.
    pub const SEMITONE: Interval = Interval::from_semitones(1);

    /// Constructs a new interval from a number of semitones.
    #[must_use]
    pub const fn from_semitones(semitones: i8) -> Interval {
        Interval { semitones }
    }

    /// Returns the number of semitones in the interval.
    #[must_use]
    pub fn semitones(self) -> i8 {
        self.semitones
    }
}
