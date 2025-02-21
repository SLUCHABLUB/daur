use crate::interval::Interval;
use std::cmp::Ordering;
use std::ops::Sub;

// TODO: microtonality?
#[derive(Copy, Clone, Debug)]
pub struct Pitch {
    from_a440: Interval,
}

impl Pitch {
    pub const A440: Pitch = Pitch {
        from_a440: Interval::PERFECT_UNISON,
    };
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
