use crate::sign::{Sign, FLAT, SHARP};
use arcstr::{literal, ArcStr};
use const_str::concat;
use strum::VariantArray;

#[expect(clippy::min_ident_chars, reason = "Chromas are named after letters")]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, VariantArray)]
pub enum Chroma {
    #[default]
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
    pub fn name(self, sign: Sign) -> ArcStr {
        match sign {
            Sign::Sharp => self.sharp_name(),
            Sign::Flat => self.flat_name(),
        }
    }

    pub fn is_black_key(self) -> bool {
        self.sharp_name() != self.flat_name()
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

    fn sharp_name(self) -> ArcStr {
        match self {
            Chroma::A => literal!("A"),
            Chroma::Bb => literal!(concat!("A", SHARP)),
            Chroma::B => literal!("B"),
            Chroma::C => literal!("C"),
            Chroma::Db => literal!(concat!("C", SHARP)),
            Chroma::D => literal!("D"),
            Chroma::Eb => literal!(concat!("D", SHARP)),
            Chroma::E => literal!("E"),
            Chroma::F => literal!("F"),
            Chroma::Gb => literal!(concat!("F", SHARP)),
            Chroma::G => literal!("G"),
            Chroma::Ab => literal!(concat!("G", SHARP)),
        }
    }

    fn flat_name(self) -> ArcStr {
        match self {
            Chroma::A => literal!("A"),
            Chroma::Bb => literal!(concat!("B", FLAT)),
            Chroma::B => literal!("B"),
            Chroma::C => literal!("C"),
            Chroma::Db => literal!(concat!("D", FLAT)),
            Chroma::D => literal!("D"),
            Chroma::Eb => literal!(concat!("E", FLAT)),
            Chroma::E => literal!("E"),
            Chroma::F => literal!("F"),
            Chroma::Gb => literal!(concat!("G", FLAT)),
            Chroma::G => literal!("G"),
            Chroma::Ab => literal!(concat!("A", FLAT)),
        }
    }
}
