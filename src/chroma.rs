use crate::sign::{Sign, FLAT, SHARP};
use const_str::concat;
use std::fmt::{Display, Formatter};
use strum::VariantArray;

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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.sharp_name(), self.flat_name())
    }
}
