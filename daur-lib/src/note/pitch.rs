use crate::note::{Chroma, Interval, Sign};
use num::Integer as _;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign, Sub};
use ux::u7;

// TODO: microtonality?
#[derive(Copy, Clone, Debug)]
/// A pitch / frequency.
pub struct Pitch {
    // TODO: use midi number
    from_a_440: Interval,
}

impl Pitch {
    /// A<sub>4</sub> (440 Hz)
    pub const A_440: Pitch = Pitch {
        from_a_440: Interval::PERFECT_UNISON,
    };

    /// Returns the croma of the pitch.
    #[must_use]
    pub fn chroma(self) -> Chroma {
        match self.from_a_440.semitones().rem_euclid(12) {
            0 => Chroma::A,
            1 => Chroma::Bb,
            2 => Chroma::B,
            3 => Chroma::C,
            4 => Chroma::Db,
            5 => Chroma::D,
            6 => Chroma::Eb,
            7 => Chroma::E,
            8 => Chroma::F,
            9 => Chroma::Gb,
            10 => Chroma::G,
            11 => Chroma::Ab,
            // unreachable
            _ => Chroma::default(),
        }
    }

    pub(crate) fn midi_number(self) -> Option<u7> {
        const A440_MIDI_NUMBER: i16 = 69;

        // TODO: open a pr for `ux` to add `From<i16>` to `u7`
        let u8: u8 = self
            .from_a_440
            .semitones()
            .saturating_add(A440_MIDI_NUMBER)
            .try_into()
            .ok()?;

        u8.try_into().ok()
    }

    fn octave_number(self) -> i16 {
        let semitones_from_c4 = self.from_a_440.semitones().saturating_add(9);
        #[expect(
            unstable_name_collisions,
            reason = "we will use the std version when it gets stabilised"
        )]
        let octaves_from_c4 = semitones_from_c4.div_floor(&12);
        octaves_from_c4.saturating_add(4)
    }

    /// Returns the name of the pitch.
    #[must_use]
    pub fn name(self, sign: Sign) -> String {
        format!("{}{}", self.chroma().name(sign), self.octave_number())
    }

    pub(crate) const fn a_440_plus(semitones: i16) -> Pitch {
        Pitch {
            from_a_440: Interval::from_semitones(semitones),
        }
    }
}

impl PartialEq for Pitch {
    fn eq(&self, other: &Self) -> bool {
        self.from_a_440.semitones() == other.from_a_440.semitones()
    }
}

impl Eq for Pitch {}

impl PartialOrd for Pitch {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Pitch {
    fn cmp(&self, other: &Self) -> Ordering {
        self.from_a_440
            .semitones()
            .cmp(&other.from_a_440.semitones())
    }
}

impl Hash for Pitch {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.from_a_440.semitones().hash(state);
    }
}

impl Add<Interval> for Pitch {
    type Output = Pitch;

    fn add(self, rhs: Interval) -> Self::Output {
        // Saturating here is fine since it is ca. 3000 octaves outside the range of the piano.
        let semitones = self.from_a_440.semitones().saturating_add(rhs.semitones());
        Pitch {
            from_a_440: Interval::from_semitones(semitones),
        }
    }
}

impl AddAssign<Interval> for Pitch {
    fn add_assign(&mut self, rhs: Interval) {
        *self = *self + rhs;
    }
}

impl Sub for Pitch {
    type Output = Interval;

    fn sub(self, rhs: Self) -> Self::Output {
        // Saturating here is fine since it is ca. 3000 octaves outside the range of the piano
        let semitones = self
            .from_a_440
            .semitones()
            .saturating_sub(rhs.from_a_440.semitones());
        Interval::from_semitones(semitones)
    }
}
