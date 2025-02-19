use crate::sign::{Sign, FLAT, SHARP};
use const_str::concat;
use std::fmt;
use std::fmt::{Display, Formatter};
use strum::VariantArray;

#[expect(clippy::min_ident_chars, reason = "Chromas are named after letters")]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, VariantArray)]
pub enum Chroma {
    A,
    Bb,
    B,
    C,
    Db,
    D,
    Eb,
    E,
    F,
    Gb,
    G,
    Ab,
}

impl Chroma {
    pub fn name(self, sign: Sign) -> &'static str {
        match sign {
            Sign::Sharp => self.sharp_name(),
            Sign::Flat => self.flat_name(),
        }
    }

    pub fn with_sign(self, sign: Sign) -> Chroma {
        #![expect(
            clippy::wildcard_enum_match_arm,
            reason = "no new chromas will be added"
        )]
        match sign {
            Sign::Sharp => match self {
                Chroma::A => Chroma::Bb,
                Chroma::C => Chroma::Db,
                Chroma::D => Chroma::Eb,
                Chroma::F => Chroma::Gb,
                Chroma::G => Chroma::Ab,
                _ => self,
            },
            Sign::Flat => match self {
                Chroma::A => Chroma::Ab,
                Chroma::B => Chroma::Bb,
                Chroma::D => Chroma::Db,
                Chroma::E => Chroma::Eb,
                Chroma::G => Chroma::Gb,
                _ => self,
            },
        }
    }

    fn sharp_name(self) -> &'static str {
        match self {
            Chroma::A => "A",
            Chroma::Bb => concat!("A", SHARP),
            Chroma::B => "B",
            Chroma::C => "C",
            Chroma::Db => concat!("C", SHARP),
            Chroma::D => "D",
            Chroma::Eb => concat!("D", SHARP),
            Chroma::E => "E",
            Chroma::F => "F",
            Chroma::Gb => concat!("F", SHARP),
            Chroma::G => "G",
            Chroma::Ab => concat!("G", SHARP),
        }
    }

    fn flat_name(self) -> &'static str {
        match self {
            Chroma::A => "A",
            Chroma::Bb => concat!("B", FLAT),
            Chroma::B => "B",
            Chroma::C => "C",
            Chroma::Db => concat!("D", FLAT),
            Chroma::D => "D",
            Chroma::Eb => concat!("E", FLAT),
            Chroma::E => "E",
            Chroma::F => "F",
            Chroma::Gb => concat!("G", FLAT),
            Chroma::G => "G",
            Chroma::Ab => concat!("A", FLAT),
        }
    }
}

impl Display for Chroma {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.sharp_name() == self.flat_name() {
            write!(f, "{}", self.sharp_name())
        } else {
            write!(f, "{}/{}", self.sharp_name(), self.flat_name())
        }
    }
}
