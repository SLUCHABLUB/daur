use crate::note::Interval;
use crate::note::PitchClass;
use crate::note::Sign;
use num::Integer as _;
use serde::Deserialize;
use serde::Serialize;
use std::hash::Hash;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize)]
/// A pitch / frequency within the MIDI range.
pub struct Pitch {
    // INVARIANT: this is non-negative
    midi_number: i8,
}

impl Pitch {
    /// The lowest pitch available in the MIDI standard: C<sub>-1</sub>.
    pub const LOWEST: Pitch = Pitch { midi_number: 0 };

    /// Returns the class of the pitch.
    #[must_use]
    pub fn class(self) -> PitchClass {
        match self.midi_number.rem_euclid(12) {
            0 => PitchClass::C,
            1 => PitchClass::Db,
            2 => PitchClass::D,
            3 => PitchClass::Eb,
            4 => PitchClass::E,
            5 => PitchClass::F,
            6 => PitchClass::Gb,
            7 => PitchClass::G,
            8 => PitchClass::Ab,
            9 => PitchClass::A,
            10 => PitchClass::Bb,
            11 => PitchClass::B,
            // unreachable
            _ => PitchClass::default(),
        }
    }

    fn octave_number(self) -> i8 {
        self.midi_number.div_floor(&12).saturating_sub(1)
    }

    /// Returns the name of the pitch.
    #[must_use]
    pub fn name(self, sign: Sign) -> String {
        format!("{}{}", self.class().name(sign), self.octave_number())
    }

    pub(crate) fn frequency(self) -> f32 {
        440.0 * 2_f32.powf((f32::from(self.midi_number) - 69.0) / 12.0)
    }
}

impl<'de> Deserialize<'de> for Pitch {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;

        let midi_number = i8::deserialize(deserializer)?;

        if midi_number < 0 {
            return Err(Error::custom("midi numbers must be non-negative"));
        }

        Ok(Pitch { midi_number })
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
