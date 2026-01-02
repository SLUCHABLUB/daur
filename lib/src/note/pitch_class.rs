use crate::note::FLAT;
use crate::note::SHARP;
use crate::note::Sign;
use arcstr::ArcStr;
use arcstr::literal;
use const_str::concat;
use enum_iterator::Sequence;
use serde::Deserialize;
use serde::Serialize;

// TODO: use `FromStr` for `Deserialize`
/// A [pitch class](https://en.wikipedia.org/wiki/Pitch_class).
#[expect(
    clippy::min_ident_chars,
    reason = "Pitch classes are named after letters"
)]
#[derive(
    Copy,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Debug,
    Default,
    Sequence,
    Serialize,
    Deserialize,
)]
pub enum PitchClass {
    #[default]
    /// A
    A,
    /// A♯ / B♭
    Bb,
    /// B
    B,
    /// C
    C,
    /// C♯ / D♭
    Db,
    /// D
    D,
    /// D♯ / E♭
    Eb,
    /// E
    E,
    /// F
    F,
    /// F♯ / G♭
    Gb,
    /// G
    G,
    /// G♯ / A♭
    Ab,
}

impl PitchClass {
    /// Returns the name of the pitch class.
    #[must_use]
    pub fn name(self, sign: Sign) -> ArcStr {
        match sign {
            Sign::Sharp => self.sharp_name(),
            Sign::Flat => self.flat_name(),
        }
    }

    /// Whether the pitch class represents a black key on the piano.
    #[must_use]
    pub fn is_black_key(self) -> bool {
        self.sharp_name() != self.flat_name()
    }

    /// Moves the pitch class by one semitone.
    #[must_use]
    pub fn with_sign(self, sign: Sign) -> PitchClass {
        #![expect(
            clippy::wildcard_enum_match_arm,
            reason = "no new pitch classes will be added"
        )]
        match sign {
            Sign::Sharp => match self {
                PitchClass::A => PitchClass::Bb,
                PitchClass::C => PitchClass::Db,
                PitchClass::D => PitchClass::Eb,
                PitchClass::F => PitchClass::Gb,
                PitchClass::G => PitchClass::Ab,
                _ => self,
            },
            Sign::Flat => match self {
                PitchClass::A => PitchClass::Ab,
                PitchClass::B => PitchClass::Bb,
                PitchClass::D => PitchClass::Db,
                PitchClass::E => PitchClass::Eb,
                PitchClass::G => PitchClass::Gb,
                _ => self,
            },
        }
    }

    fn sharp_name(self) -> ArcStr {
        match self {
            PitchClass::A => literal!("A"),
            PitchClass::Bb => literal!(concat!("A", SHARP)),
            PitchClass::B => literal!("B"),
            PitchClass::C => literal!("C"),
            PitchClass::Db => literal!(concat!("C", SHARP)),
            PitchClass::D => literal!("D"),
            PitchClass::Eb => literal!(concat!("D", SHARP)),
            PitchClass::E => literal!("E"),
            PitchClass::F => literal!("F"),
            PitchClass::Gb => literal!(concat!("F", SHARP)),
            PitchClass::G => literal!("G"),
            PitchClass::Ab => literal!(concat!("G", SHARP)),
        }
    }

    fn flat_name(self) -> ArcStr {
        match self {
            PitchClass::A => literal!("A"),
            PitchClass::Bb => literal!(concat!("B", FLAT)),
            PitchClass::B => literal!("B"),
            PitchClass::C => literal!("C"),
            PitchClass::Db => literal!(concat!("D", FLAT)),
            PitchClass::D => literal!("D"),
            PitchClass::Eb => literal!(concat!("E", FLAT)),
            PitchClass::E => literal!("E"),
            PitchClass::F => literal!("F"),
            PitchClass::Gb => literal!(concat!("G", FLAT)),
            PitchClass::G => literal!("G"),
            PitchClass::Ab => literal!(concat!("A", FLAT)),
        }
    }
}
