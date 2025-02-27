use crate::chroma::Chroma;
use crate::interval::Interval;
use crate::sign::Sign;
use num::Integer as _;
use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Sub};

// TODO: microtonality?
#[derive(Copy, Clone, Debug)]
pub struct Pitch {
    from_a440: Interval,
}

impl Pitch {
    pub const A440: Pitch = Pitch {
        from_a440: Interval::PERFECT_UNISON,
    };

    pub fn chroma(self) -> Chroma {
        match self.from_a440.semitones().rem_euclid(12) {
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
            _ => Chroma::default(),
        }
    }

    fn octave_number(self) -> i16 {
        let semitones_from_c4 = self.from_a440.semitones().saturating_add(9);
        #[expect(
            unstable_name_collisions,
            reason = "we will use the std version when it gets stabilised"
        )]
        let octaves_from_c4 = semitones_from_c4.div_floor(&12);
        octaves_from_c4.saturating_add(4)
    }

    pub fn name(self, sign: Sign) -> String {
        format!("{}{}", self.chroma().name(sign), self.octave_number())
    }
}

impl Add<Interval> for Pitch {
    type Output = Pitch;

    fn add(self, rhs: Interval) -> Self::Output {
        // Saturating here is fine since it's like 3000 octaves outside of piano range
        let semitones = self.from_a440.semitones().saturating_add(rhs.semitones());
        Pitch {
            from_a440: Interval::from_semitones(semitones),
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
        // Saturating here is fine since it's like 3000 octaves outside of piano range
        let semitones = self
            .from_a440
            .semitones()
            .saturating_sub(rhs.from_a440.semitones());
        Interval::from_semitones(semitones)
    }
}

impl PartialEq for Pitch {
    fn eq(&self, other: &Self) -> bool {
        self.from_a440.semitones() == other.from_a440.semitones()
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
        self.from_a440.semitones().cmp(&other.from_a440.semitones())
    }
}
