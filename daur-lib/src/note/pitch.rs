use crate::note::{Chroma, Interval, Sign};
use num::Integer as _;
use std::hash::Hash;
use std::ops::{Add, AddAssign, Sub};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
/// A pitch / frequency within the MIDI range.
pub struct Pitch {
    // INVARIANT: this is non-negative
    midi_number: i8,
}

impl Pitch {
    /// The lowest pitch available in the MIDI standard: C<sub>-1</sub>.
    pub const LOWEST: Pitch = Pitch { midi_number: 0 };

    pub(crate) fn midi_number(self) -> u8 {
        #![expect(clippy::cast_sign_loss, reason = "see the invariant")]
        self.midi_number as u8
    }

    /// Returns the croma of the pitch.
    #[must_use]
    pub fn chroma(self) -> Chroma {
        match self.midi_number.rem_euclid(12) {
            0 => Chroma::C,
            1 => Chroma::Db,
            2 => Chroma::D,
            3 => Chroma::Eb,
            4 => Chroma::E,
            5 => Chroma::F,
            6 => Chroma::Gb,
            7 => Chroma::G,
            8 => Chroma::Ab,
            9 => Chroma::A,
            10 => Chroma::Bb,
            11 => Chroma::B,
            // unreachable
            _ => Chroma::default(),
        }
    }

    fn octave_number(self) -> i8 {
        self.midi_number.div_floor(&12).saturating_sub(1)
    }

    /// Returns the name of the pitch.
    #[must_use]
    pub fn name(self, sign: Sign) -> String {
        format!("{}{}", self.chroma().name(sign), self.octave_number())
    }
}

impl Add<Interval> for Pitch {
    type Output = Pitch;

    fn add(mut self, rhs: Interval) -> Pitch {
        self += rhs;
        self
    }
}

impl AddAssign<Interval> for Pitch {
    fn add_assign(&mut self, rhs: Interval) {
        self.midi_number = self.midi_number.saturating_add(rhs.semitones());

        if self.midi_number.is_negative() {
            self.midi_number = 0;
        }
    }
}

impl Sub for Pitch {
    type Output = Interval;

    fn sub(self, rhs: Pitch) -> Interval {
        let semitones = self.midi_number.saturating_sub(rhs.midi_number);
        Interval::from_semitones(semitones)
    }
}
